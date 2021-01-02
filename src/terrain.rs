//! A full world is composed of 12 chunks in each direction.
//! Thus, the world is 144 chunks in area, or 1728 chunks in volume.
//! Each chunk is 64 voxels wide, which makes it 4096 voxels in area, or
//! 262_144 voxels in volume.
//! The world in total is 12 * 64 = 768 voxels long, 589_824 voxels in area,
//! and 452_984_832 voxels in volume.
//!
//! The origin point (0, 0, 0) is in the middle of the map. This means that
//! the minimum point on the map is (-384, -384, -384) and the maximum point
//! is (384, 384, 384).
use std::collections::HashMap;

use bevy::{ecs::Query, render::pipeline::PrimitiveTopology};
use bevy::{
    ecs::{Commands, Entity, IntoSystem, Res, ResMut},
    input::Input,
    pbr::PbrBundle,
    prelude::{AppBuilder, Assets, Color, Handle, KeyCode, Mesh, Plugin, StandardMaterial},
    reflect::TypeUuid,
    render::{
        mesh::{Indices, VertexAttributeValues},
        renderer::RenderResources,
    },
};
use bevy::{log::trace, prelude::Visible};
use bevy::{
    prelude::{AddAsset, AssetServer, RenderPipelines, Shader},
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::ShaderStages,
    },
    tasks::ComputeTaskPool,
};
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::Point};
use building_blocks::{
    core::Point3,
    mesh::IsOpaque,
    storage::{Array3, ForEach, Snappy},
};
use building_blocks::{
    core::{Extent3i, Point2i, Point3i, PointN},
    storage::Get,
};
use building_blocks::{
    mesh::{greedy_quads, padded_greedy_quads_chunk_extent, GreedyQuadsBuffer, PosNormMesh},
    storage::LocalChunkCache,
};
use building_blocks::{
    prelude::{copy_extent, LocalChunkCache3},
    storage::{
        ChunkMapBuilder, ChunkMapBuilder3, CompressibleChunkMap3, CompressibleChunkStorage,
        IterChunkKeys,
    },
};
use colonize_noise::Noise2d;
use noise::{MultiFractal, RidgedMulti, Seedable};
use rand::{thread_rng, Rng};

use colonize_common::CubeVoxel;

const CHUNK_SIZE: usize = 32;
const REGION_SIZE: usize = 128; // CHUNK_SIZE * NUM_CHUNKS
                                // 512 underground blocks, plus 256 blocks above sea level.
const REGION_HEIGHT: i32 = 128;
const REGION_MIN_3D: Point3i = PointN([-(REGION_SIZE as i32 / 2), -64, -(REGION_SIZE as i32 / 2)]);
const REGION_SHAPE_3D: Point3i = PointN([REGION_SIZE as i32, REGION_HEIGHT, REGION_SIZE as i32]);
const REGION_MAX_3D: Point3i = PointN([
    REGION_MIN_3D.0[0] + REGION_SHAPE_3D.0[0],
    REGION_MIN_3D.0[1] + REGION_SHAPE_3D.0[1],
    REGION_MIN_3D.0[2] + REGION_SHAPE_3D.0[2],
]);

const SEA_LEVEL: i32 = 0;

const DEFAULT_BUILDER: ChunkMapBuilder3<CubeVoxel> = ChunkMapBuilder {
    chunk_shape: PointN([CHUNK_SIZE as i32; 3]),
    ambient_value: CubeVoxel::Air,
    default_chunk_metadata: (),
};

pub(crate) const TERRAIN: &str = "terrain";

#[derive(Debug)]
pub struct Chunk;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<MeshMaterial>()
            .add_resource(TerrainResource::default())
            .add_resource(MeshResource::default())
            .add_startup_system(setup.system())
            .add_startup_system_to_stage(TERRAIN, generate_voxels.system())
            .add_system(generate_meshes.system())
            .add_system(hide_y_levels_system.system())
            .add_system(modify_config.system());
    }
}

fn setup(
    mut res: ResMut<TerrainResource>,
    mut materials: ResMut<Assets<MeshMaterial>>,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    res.materials.insert(
        CubeVoxel::Stone,
        materials.add(MeshMaterial {
            color: Color::rgb(0.5, 0.5, 0.5).into(), // Stone
        }),
    );
    res.materials.insert(
        CubeVoxel::Grass,
        materials.add(MeshMaterial {
            color: Color::rgb(0.376, 0.502, 0.22).into(), // Grass
        }),
    );
    res.materials.insert(
        CubeVoxel::Gold,
        materials.add(MeshMaterial {
            color: Color::rgb(1.0, 0.843, 0.).into(), // Gold
        }),
    );
    res.materials.insert(
        CubeVoxel::Water,
        materials.add(MeshMaterial {
            color: Color::rgba(0.0, 0.0, 0.5, 0.5).into(), // Water
        }),
    );

    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    // Create a new shader pipeline with shaders loaded from the asset directory
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/hot.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/hot.frag")),
    }));
    res.pipeline_handle = Some(pipeline_handle);

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MeshMaterial resources to our shader
    render_graph.add_system_node(
        "mesh_material",
        AssetRenderResourcesNode::<MeshMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "mesh_material" node to the main pass node. This ensures "mesh_material" runs before the main pass
    render_graph
        .add_node_edge("mesh_material", base::node::MAIN_PASS)
        .unwrap();
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct YLevel(i32);

impl YLevel {
    fn increment(&mut self) {
        self.0 = i32::min(self.0 + 1, REGION_MAX_3D.y());
        trace!("Incremented y-level to {:?}", self.0);
    }

    fn decrement(&mut self) {
        self.0 = i32::max(self.0 - 1, REGION_MIN_3D.y());
        trace!("Decremented y-level to {:?}", self.0);
    }
}

impl Default for YLevel {
    fn default() -> Self {
        Self(REGION_MAX_3D.y())
    }
}

pub(crate) struct TerrainResource {
    materials: HashMap<CubeVoxel, Handle<MeshMaterial>>,
    noise: RidgedMulti,
    chunks: CompressibleChunkMap3<CubeVoxel>,
    generated_voxels: bool,
    sea_level: f64,
    y_offset: f64,
    y_level: YLevel,
    pipeline_handle: Option<Handle<PipelineDescriptor>>,
}

impl TerrainResource {
    pub(crate) fn surface_y(&self, column: Point2i) -> i32 {
        let local_cache = LocalChunkCache::new();
        let reader = self.chunks.storage().reader(&local_cache);
        let reader_map = DEFAULT_BUILDER.build_with_read_storage(reader);
        let bounding_extent = reader_map.bounding_extent();
        let min_y = bounding_extent.minimum.y();
        for y in (min_y..bounding_extent.max().y()).rev() {
            let location = PointN([column.x(), y, column.y()]);
            let value = reader_map.get(&location);
            if value != CubeVoxel::Air {
                return y;
            }
        }
        min_y
    }

    pub(crate) fn find_nearest_gold(&self, x: i32, y: i32, z: i32) -> Option<Point3<i32>> {
        const SEARCH_SIZE: i32 = 10;
        let min = PointN([x, y, z]) + PointN([-(SEARCH_SIZE as i32 / 2); 3]);
        let size = PointN([SEARCH_SIZE as i32; 3]);
        let query_extent = Extent3i::from_min_and_shape(min, size);
        let mut gold_loc = Option::None;
        let local_cache = LocalChunkCache::new();
        let reader = self.chunks.storage().reader(&local_cache);
        let reader_map = DEFAULT_BUILDER.build_with_read_storage(reader);
        //for chunk_key in self.chunks.indexer.chunk_keys_for_extent(&query_extent) {
        //    if let Some(chunk) = reader.get(&chunk_key) {
        //        chunk.array.for_each(&query_extent.clone(), |p: Point3i, value| f(p, value));
        //    } else {
        //        let chunk_extent = self.chunks.indexer.extent_for_chunk_at_key(chunk_key);
        //        AmbientExtent::new(self.chunks.ambient_value())
        //            .for_each(&query_extent.intersection(&chunk_extent), |p, value| f(p, value));
        //    }
        //}
        let f = |p: Point3i, value| {
            if value == CubeVoxel::Gold {
                gold_loc.replace(p);
            }
        };
        reader_map.for_each(&query_extent, f);
        gold_loc
    }
}

impl Default for TerrainResource {
    fn default() -> Self {
        let store = CompressibleChunkStorage::new(Snappy);
        Self {
            materials: HashMap::new(),
            //noise: Fbm::new().set_frequency(0.008).set_octaves(8),
            noise: RidgedMulti::new()
                .set_frequency(0.001)
                .set_lacunarity(4.0)
                .set_persistence(0.7)
                .set_octaves(8),
            chunks: DEFAULT_BUILDER.build_with_write_storage(store),
            generated_voxels: false,
            sea_level: 100.,
            y_offset: 10.,
            y_level: YLevel::default(),
            pipeline_handle: None,
        }
    }
}

struct MeshResource {
    meshes: HashMap<Point3i, Vec<(Entity, Handle<Mesh>)>>,
}

impl Default for MeshResource {
    fn default() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }
}

#[derive(Default, RenderResources, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
pub struct MeshMaterial {
    color: Color,
}

fn modify_config(
    commands: &mut Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut terrain_res: ResMut<TerrainResource>,
    mut mesh_res: ResMut<MeshResource>,
) {
    let mut reset_flag = false;

    // Increase/decrease the sea level by 10 if the player pressed `Y` or `G`.
    if keyboard_input.pressed(KeyCode::Y) {
        terrain_res.sea_level += 10.;
        reset_flag = true;
    } else if keyboard_input.pressed(KeyCode::G) {
        terrain_res.sea_level -= 10.;
        reset_flag = true;
    }

    // Increase/decrease the y-offset by 1 if the player pressed `U` or `H`.
    if keyboard_input.pressed(KeyCode::U) {
        terrain_res.y_offset += 1.;
        reset_flag = true;
    } else if keyboard_input.pressed(KeyCode::H) {
        terrain_res.y_offset -= 1.;
        reset_flag = true;
    }

    // Increase/decrease the frequency by 0.001 if the player pressed `I` or `J`.
    if keyboard_input.pressed(KeyCode::I) {
        terrain_res.noise.frequency += 0.001;
        reset_flag = true;
    } else if keyboard_input.pressed(KeyCode::J) {
        terrain_res.noise.frequency -= 0.001;
        reset_flag = true;
    }

    // Increase/decrease the lacunarity by 0.1 if the player pressed `O` or `K`.
    if keyboard_input.pressed(KeyCode::O) {
        terrain_res.noise.lacunarity += 0.1;
        reset_flag = true;
    } else if keyboard_input.pressed(KeyCode::K) {
        terrain_res.noise.lacunarity -= 0.1;
        reset_flag = true;
    }

    // Increase/decrease the persistence by 0.1 if the player pressed `P` or `L`.
    if keyboard_input.pressed(KeyCode::P) {
        terrain_res.noise.persistence += 0.1;
        reset_flag = true;
    } else if keyboard_input.pressed(KeyCode::L) {
        terrain_res.noise.persistence -= 0.1;
        reset_flag = true;
    }

    if reset_flag {
        reset_world(commands, &mut mesh_assets, &mut terrain_res, &mut mesh_res);
    }
}

fn hide_y_levels_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut terrain_res: ResMut<TerrainResource>,
    mesh_res: ResMut<MeshResource>,
    mut mesh_query: Query<(Option<&FullDetailMesh>, &YLevel, &mut Visible)>,
) {
    // Increase/decrease the Y-level by 1 if the player pressed `<` or `>`.
    let old_y_level = terrain_res.y_level;
    if keyboard_input.pressed(KeyCode::Minus) {
        terrain_res.y_level.decrement();
    } else if keyboard_input.pressed(KeyCode::Equals) {
        terrain_res.y_level.increment();
    }

    if old_y_level != terrain_res.y_level {
        // Find and disable all the meshes at the old y-level.
        for (chunk_pos, mesh_entities) in &mesh_res.meshes {
            for (entity, _mesh) in mesh_entities {
                if let Ok((_, y_level, mut visible)) = mesh_query.get_mut(*entity) {
                    if y_level == &old_y_level {
                        trace!(
                            "Disabled mesh at y-level {:?} for chunk {:?}",
                            y_level,
                            chunk_pos
                        );
                        visible.is_visible = false;
                    }
                }
            }
        }

        // Find and enable all the meshes at the current y-level (that aren't full-detail meshes).
        for (chunk_pos, mesh_entities) in &mesh_res.meshes {
            for (entity, _mesh) in mesh_entities {
                if let Ok((None, y_level, mut visible)) = mesh_query.get_mut(*entity) {
                    if y_level == &terrain_res.y_level {
                        trace!(
                            "Enabled mesh at y-level {:?} for chunk {:?}",
                            y_level,
                            chunk_pos
                        );
                        visible.is_visible = true;
                    }
                }
            }
        }

        // If the previous y-level was the top of a chunk, and we went up over a chunk boundary,
        // go back and re-enable the full-detail meshes for that chunk.
        for (chunk_pos, mesh_entities) in &mesh_res.meshes {
            if old_y_level.0 == chunk_pos.y() + CHUNK_SIZE as i32 - 1
                && terrain_res.y_level.0 == chunk_pos.y() + CHUNK_SIZE as i32
            {
                for (entity, _mesh) in mesh_entities {
                    if let Ok((Some(_), y_level, mut visible)) = mesh_query.get_mut(*entity) {
                        if y_level == &old_y_level {
                            trace!(
                                "Re-enabled mesh at y-level {:?} for chunk {:?}",
                                y_level,
                                chunk_pos
                            );
                            visible.is_visible = true;
                        }
                    }
                }
            }
        }
    }
}

/// Removes all voxels & meshes and marks the world for regeneration.
fn reset_world(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    terrain_res: &mut ResMut<TerrainResource>,
    mesh_res: &mut ResMut<MeshResource>,
) {
    // Delete the voxels associated with the current world.
    let store = CompressibleChunkStorage::new(Snappy);
    terrain_res.chunks = DEFAULT_BUILDER.build_with_write_storage(store);

    // Delete the entities and meshes associated with the current world.
    let to_remove = mesh_res.meshes.keys().cloned().collect::<Vec<_>>();
    for p in to_remove {
        if let Some(meshes) = mesh_res.meshes.remove(&p) {
            for (entity, mesh) in meshes {
                commands.despawn(entity);
                mesh_assets.remove(&mesh);
            }
        }
    }

    // Mark the world as ungenerated.
    terrain_res.generated_voxels = false;
}

fn generate_voxels(mut terrain_res: ResMut<TerrainResource>) {
    if terrain_res.generated_voxels {
        return;
    }

    // Generate the 3D voxel map of the terrain.
    let elevation_noise = Noise2d::new(
        RidgedMulti::new()
            .set_seed(random_seed())
            .set_frequency(0.001)
            .set_lacunarity(4.0)
            .set_persistence(0.7)
            .set_octaves(8),
    );
    let dirt_thickness_noise = Noise2d::new(RidgedMulti::new().set_seed(random_seed()));
    let query = Extent3i::from_min_and_shape(REGION_MIN_3D, REGION_SHAPE_3D);
    trace!("Generating 3D strata map for extent {:?}", query);
    let strata_array = colonize_core::generate_map(
        &elevation_noise,
        &dirt_thickness_noise,
        SEA_LEVEL,
        REGION_MIN_3D,
        REGION_SHAPE_3D,
    );

    // Copy over the voxels from their intermediate representations to the chunk map.
    trace!("Copying chunk data to chunk map");
    copy_extent(&query, &strata_array, &mut terrain_res.chunks);
    trace!("Finished generating the world");

    terrain_res.generated_voxels = true;
}

fn random_seed() -> u32 {
    thread_rng().gen()
}

fn generate_meshes(
    commands: &mut Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    terrain: Res<TerrainResource>,
    mut mesh_res: ResMut<MeshResource>,
    pool: Res<ComputeTaskPool>,
) {
    let map_ref = &terrain.chunks;
    let chunk_keys = map_ref
        .storage()
        .chunk_keys()
        .into_iter()
        .filter(|k| {
            // If the mesh exists for this chunk, then there's nothing to do.
            mesh_res.meshes.get(&k).is_none()
        })
        .collect::<Vec<_>>();
    let meshes = (&pool.0).scope(|s| {
        for chunk_key in chunk_keys {
            s.spawn(generate_mesh(map_ref, chunk_key))
        }
    });
    for chunk in meshes.into_iter() {
        if let (p, Some(layers_map)) = chunk {
            let entities = layers_map
                .into_iter()
                .flat_map(|((y_level, full_detail), material_meshes)| {
                    material_meshes
                        .into_iter()
                        .map(move |(material, mesh)| (y_level, full_detail, material, mesh))
                })
                .map(|(y_level, full_detail, material, mesh)| {
                    generate_mesh_entity(
                        mesh,
                        // Only treat the mesh as collidable if it's the "full detail" mesh for the chunk.
                        // The rest are for rendering only.
                        full_detail && material.collidable(),
                        commands,
                        terrain.pipeline_handle.as_ref().unwrap().clone(),
                        terrain.materials.get(&material).unwrap().clone(),
                        !material.is_opaque(),
                        &mut mesh_assets,
                        y_level,
                        full_detail,
                    )
                })
                .collect::<Vec<_>>();
            mesh_res.meshes.insert(p, entities);
        } else if let (p, None) = chunk {
            // Insert points with no associated mesh into the hash map.
            // We use the presence of the chunk key in the hash map as a flag on
            // whether or not to generate the mesh. Chunks without meshes (i.e.
            // chunks with just air) shouldn't be regenerated so we add them to
            // the hash map as well.
            mesh_res.meshes.insert(p, Vec::new());
        }
    }
}

struct FullDetailMesh;

async fn generate_mesh(
    map_ref: &CompressibleChunkMap3<CubeVoxel>,
    chunk_key: &Point3i,
) -> (
    Point3i,
    Option<HashMap<(YLevel, bool), HashMap<CubeVoxel, PosNormMesh>>>,
) {
    trace!("Generating mesh for chunk at {:?}", chunk_key);
    let local_cache = LocalChunkCache3::new();
    let chunk_extent = map_ref.indexer.extent_for_chunk_at_key(*chunk_key);
    let padded_chunk_extent = padded_greedy_quads_chunk_extent(&chunk_extent);

    // Create a mesh for each possible size of the chunk.
    let mut padded_layer_extent = padded_chunk_extent;
    *padded_layer_extent.shape.y_mut() = 3;
    let mut padded_layer_extents = Vec::new();
    while padded_layer_extent.max().y() <= padded_chunk_extent.max().y() {
        padded_layer_extents.push(padded_layer_extent);
        *padded_layer_extent.shape.y_mut() += 1;
    }
    trace!(
        "Generating {} padded layers for chunk {:?}",
        padded_layer_extents.len(),
        chunk_key
    );

    let reader = map_ref.storage().reader(&local_cache);
    let reader_map = DEFAULT_BUILDER.build_with_read_storage(reader);
    let mut layer_meshes: HashMap<(YLevel, bool), HashMap<CubeVoxel, PosNormMesh>> = HashMap::new();
    // Iterate over the slice extents in reverse order. The first extent will be the "full" extent of the
    // chunk. This is a special case because we want to generate both a sliced (i.e. "pretend there's only air
    // above us") mesh and a full-detail one (no air padding above).
    let mut extents = Vec::new();
    for padded_layer_extent in padded_layer_extents.into_iter().rev() {
        if extents.is_empty() {
            // For the first (full size) extent, generate a mesh for it twice. Once with a layer of
            // air padding, and once without. We need both versions for if we're viewing the top of the mesh
            // or if the top of the mesh is obscured by the chunk above it.
            extents.push((true, padded_layer_extent, padded_layer_extent));
        }
        extents.push((
            false,
            padded_layer_extent,
            padded_layer_extent.add_to_shape(PointN([0, -1, 0])),
        ));
    }

    for (full_detail, padded_layer_extent, extent_to_copy) in extents {
        trace!(
            "Copying extent {:?} for layer {:?}",
            extent_to_copy,
            padded_layer_extent
        );
        let mut padded_layer = Array3::fill(padded_layer_extent, CubeVoxel::Air);
        copy_extent(&extent_to_copy, &reader_map, &mut padded_layer);

        // TODO bevy: we could avoid re-allocating the buffers on every call if we had
        // thread-local storage accessible from this task
        let mut buffer = GreedyQuadsBuffer::new(padded_layer_extent);
        greedy_quads(&padded_layer, &padded_layer_extent, &mut buffer);

        // Separate the meshes by material, so that we can render each voxel type with a different color.
        let mut meshes: HashMap<CubeVoxel, PosNormMesh> = HashMap::new();
        for group in buffer.quad_groups.iter() {
            for quad in group.quads.iter() {
                let material = reader_map.get(&quad.minimum);
                let mesh = meshes.entry(material).or_insert_with(PosNormMesh::default);
                group.face.add_quad_to_pos_norm_mesh(&quad, mesh);
            }
        }

        // If all the meshes are empty, don't return anything.
        let layer_is_empty = meshes.iter().fold(
            false,
            |acc, (_material, mesh)| if acc { acc } else { mesh.is_empty() },
        );

        if layer_is_empty {
            // If the layer is empty, don't bother inserting the mesh (just discard it), and we can also
            // break out of the loop since if a bigger cut of the chunk was empty then any subsets of that
            // cut will also be empty.
            break;
        }

        layer_meshes.insert(
            (YLevel(padded_layer_extent.max().y() - 1), full_detail),
            meshes,
        );
    }

    // If all the meshes of all the layers are empty, don't return anything.
    if layer_meshes.is_empty() {
        (*chunk_key, None)
    } else {
        (*chunk_key, Some(layer_meshes))
    }
}

fn generate_mesh_entity(
    mesh: PosNormMesh,
    collidable: bool,
    commands: &mut Commands,
    pipeline_handle: Handle<PipelineDescriptor>,
    material: Handle<MeshMaterial>,
    is_transparent: bool,
    meshes: &mut Assets<Mesh>,
    y_level: YLevel,
    full_detail: bool,
) -> (Entity, Handle<Mesh>) {
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    let num_vertices = mesh.positions.len();

    let rigid_body;
    let collider;
    if collidable {
        // Generate a colliding entity for the mesh to use for physics.
        // TODO: does this position actually need to match the position of the chunk mesh?
        //   It appears to work fine without any translation applied.
        rigid_body = Some(RigidBodyBuilder::new_static());
        collider = Some(ColliderBuilder::trimesh(
            nested_array_f32_to_points(&mesh.positions),
            flat_array_f32_to_points(&mesh.indices),
        ));
    } else {
        rigid_body = None;
        collider = None;
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    render_mesh.set_attribute(
        "Vertex_Uv",
        VertexAttributeValues::Float2(vec![[0.0; 2]; num_vertices]),
    );
    // TODO: find a way to avoid this usize -> u32 conversion
    render_mesh.set_indices(Some(Indices::U32(
        mesh.indices.into_iter().map(|i| i as u32).collect(),
    )));

    let mesh_handle = meshes.add(render_mesh);

    let mut commands = commands
        .spawn(PbrBundle {
            mesh: mesh_handle.clone_weak(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            visible: Visible {
                // Only the full-detail meshes of each chunk are visible to start with.
                // Slices of chunks become visible as the player lowers the y-level.
                is_visible: full_detail,
                is_transparent,
            },
            ..Default::default()
        })
        .with(material)
        .with(Chunk)
        .with(y_level);
    if full_detail {
        commands = commands.with(FullDetailMesh);
    }
    let entity = commands.current_entity().unwrap();
    if let (Some(rigid_body), Some(mut collider)) = (rigid_body, collider) {
        collider = collider.user_data(entity.to_bits() as u128);
        commands.insert(entity, (rigid_body, collider));
    }
    (entity, mesh_handle)
}

fn nested_array_f32_to_points(array: &[[f32; 3]]) -> Vec<Point<f32>> {
    array
        .iter()
        .map(|[x, y, z]| Point::new(*x, *y, *z))
        .collect()
}

fn flat_array_f32_to_points(array: &[u32]) -> Vec<Point<u32>> {
    array
        .chunks(3)
        .map(|c| Point::new(c[0], c[1], c[2]))
        .collect()
}
