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

use bevy::pbr::PbrBundle;
use bevy::{ecs::Query, render::pipeline::PrimitiveTopology};
use bevy::{
    ecs::{Commands, Entity, IntoSystem, Res, ResMut},
    input::Input,
    prelude::{AppBuilder, Assets, Color, Handle, KeyCode, Mesh, Plugin},
    reflect::TypeUuid,
    render::{
        mesh::{Indices, VertexAttributeValues},
        renderer::RenderResources,
    },
};
use bevy::{log::trace, prelude::Visible};
use bevy::{
    prelude::AddAsset,
    render::{pipeline::PipelineDescriptor, render_graph::RenderGraph},
    tasks::ComputeTaskPool,
};
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::Point};
use building_blocks::{core::Point3, mesh::{IsOpaque, SurfaceNetsBuffer, surface_nets}, storage::{Array, Array3, ChunkMap, CompressibleChunkStorageReader, ForEach, GetUncheckedRelease, IsEmpty, Local, Snappy, Stride, TransformMap}};
use building_blocks::{
    core::{Extent3i, Point2i, Point3i, PointN, Neighborhoods},
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
use colonize_pbr::{pbr_bundle, prelude::StandardMaterial, YLevel};
use noise::{MultiFractal, RidgedMulti, Seedable};
use rand::{thread_rng, Rng};

use colonize_common::{EMPTY_VOXEL, NUM_VOXEL_TYPES, Voxel, VoxelType};

const CHUNK_SIZE: usize = 128;
const REGION_SIZE: usize = 2048; // CHUNK_SIZE * NUM_CHUNKS
                                // 512 underground blocks, plus 256 blocks above sea level.
const REGION_HEIGHT: i32 = 256;
const REGION_MIN_3D: Point3i = PointN([-(REGION_SIZE as i32 / 2), -128, -(REGION_SIZE as i32 / 2)]);
const REGION_SHAPE_3D: Point3i = PointN([REGION_SIZE as i32, REGION_HEIGHT, REGION_SIZE as i32]);
const REGION_MAX_3D: Point3i = PointN([
    REGION_MIN_3D.0[0] + REGION_SHAPE_3D.0[0],
    REGION_MIN_3D.0[1] + REGION_SHAPE_3D.0[1],
    REGION_MIN_3D.0[2] + REGION_SHAPE_3D.0[2],
]);

const SEA_LEVEL: i32 = 0;

const DEFAULT_BUILDER: ChunkMapBuilder3<Voxel> = ChunkMapBuilder {
    chunk_shape: PointN([CHUNK_SIZE as i32; 3]),
    ambient_value: EMPTY_VOXEL,
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
            .add_resource(YLevel {
                value: REGION_MAX_3D.y(),
            })
            .add_startup_system(setup.system())
            .add_startup_system_to_stage(TERRAIN, generate_voxels.system())
            .add_system(generate_meshes.system())
            .add_system(hide_y_levels_system.system())
            .add_system(modify_config.system());
    }
}

fn setup(
    mut res: ResMut<TerrainResource>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    _mesh_materials: ResMut<Assets<MeshMaterial>>,
    _pipelines: ResMut<Assets<PipelineDescriptor>>,
    _render_graph: ResMut<RenderGraph>,
) {
    for (voxel_type, color) in &[
        // Technically we don't use the "air" material ever, since air is transparent, but we still need it to
        // ensure these indices match up with the ones provided by VoxelType::index.
        (VoxelType::Air, Color::rgb(0.5, 0.5, 0.5)),
        (VoxelType::Stone, Color::rgb(0.5, 0.5, 0.5)),
        (VoxelType::Grass, Color::rgb(0.376, 0.502, 0.22)),
        (VoxelType::Gold, Color::rgb(1.0, 0.843, 0.)),
        (VoxelType::Water, Color::rgba(0.0, 0.0, 0.5, 0.5)),
    ] {
        res.materials.insert(
            *voxel_type,
            (
                standard_materials.add((*color).into()),
                HatMaterial(standard_materials.add((*color).into())),
            ),
        );
    }
}

pub(crate) struct TerrainResource {
    materials: HashMap<VoxelType, (Handle<StandardMaterial>, HatMaterial)>,
    noise: RidgedMulti,
    chunks: CompressibleChunkMap3<Voxel>,
    generated_voxels: bool,
    sea_level: f64,
    y_offset: f64,
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
            if *value.voxel_type() != VoxelType::Air {
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
        let f = |p: Point3i, value: Voxel| {
            if *value.voxel_type() == VoxelType::Gold {
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
            noise: RidgedMulti::new()
                .set_frequency(0.001)
                .set_lacunarity(4.0)
                .set_persistence(0.7)
                .set_octaves(8),
            chunks: DEFAULT_BUILDER.build_with_write_storage(store),
            generated_voxels: false,
            sea_level: 100.,
            y_offset: 10.,
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

#[derive(Clone, Default)]
pub struct HatMaterial(pub Handle<StandardMaterial>);

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
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    terrain_res: ResMut<TerrainResource>,
    mut mesh_res: ResMut<MeshResource>,
    mesh_query: Query<(Entity, Option<&FullDetailMesh>, &YLevel)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut y_level: ResMut<YLevel>,
) {
    // Increase/decrease the Y-level by 1 if the player pressed `<` or `>`.
    let old_y_level = *y_level;
    if keyboard_input.pressed(KeyCode::Minus) {
        y_level.value = i32::max(y_level.value - 1, REGION_MIN_3D.y());
        trace!("Decremented y-level to {:?}", y_level.value);
    } else if keyboard_input.pressed(KeyCode::Equals) {
        y_level.value = i32::min(y_level.value + 1, REGION_MAX_3D.y());
        trace!("Incremented y-level to {:?}", y_level.value);
    }

    if old_y_level != *y_level {
        // Find and disable all the meshes at the old y-level.
        let mut to_remove = Vec::new();
        for (_chunk_pos, mesh_entities) in mesh_res.meshes.iter() {
            for (entity, _mesh) in mesh_entities {
                if let Ok((mesh_entity, None, mesh_y_level)) = mesh_query.get(*entity) {
                    if mesh_y_level != &*y_level {
                        to_remove.push(mesh_entity);
                    }
                }
            }
        }
        for entity in to_remove {
            commands.despawn(entity);
        }

        let local_cache = LocalChunkCache3::new();
        for (chunk_pos, mesh_entities) in mesh_res.meshes.iter_mut() {
            // Generate a new mesh for the current y-level for each chunk.
            if y_level.value < chunk_pos.y() || y_level.value > chunk_pos.y() + CHUNK_SIZE as i32 {
                // If the y-level does not intersect this chunk, there's nothing to generate.
                continue;
            }
            let chunk_extent = terrain_res
                .chunks
                .indexer
                .extent_for_chunk_at_key(*chunk_pos);
            let padded_chunk_extent = padded_greedy_quads_chunk_extent(&chunk_extent);
            let mut padded_layer_extent = padded_chunk_extent;
            *padded_layer_extent.shape.y_mut() = 3;
            *padded_layer_extent.minimum.y_mut() = y_level.value - 1;
            let extent_to_copy = padded_layer_extent.add_to_shape(PointN([0, -1, 0]));
            trace!(
                "Generating padded layers for extent {:?} for chunk {:?}",
                padded_layer_extent,
                chunk_pos
            );
            let meshes = generate_mesh_for_extent_with_surface_nets(
                &terrain_res.chunks,
                &chunk_pos,
                &local_cache,
                padded_layer_extent,
                &extent_to_copy,
            );
            if let Some(meshes) = meshes {
                for (material, pos_norm_mesh) in meshes {
                    let (entity, mesh) = generate_mesh_entity(
                        pos_norm_mesh,
                        // Y-level meshes are not collidable. They're for rendering only.
                        false,
                        commands,
                        terrain_res
                            .materials
                            .get(&material)
                            .expect("failed to get material")
                            .clone(),
                        !material.is_opaque(),
                        &mut mesh_assets,
                        *y_level,
                        false,
                    );
                    mesh_entities.push((entity, mesh));
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
                        terrain
                            .materials
                            .get(&material)
                            .expect("failed to get material")
                            .clone(),
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
    map_ref: &CompressibleChunkMap3<Voxel>,
    chunk_key: &Point3i,
) -> (
    Point3i,
    Option<HashMap<(YLevel, bool), HashMap<VoxelType, PosNormMesh>>>,
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
        *padded_layer_extent.minimum.y_mut() += 1;
    }
    padded_layer_extents.push(padded_chunk_extent);
    trace!(
        "Generating {} padded layers for chunk {:?}",
        padded_layer_extents.len(),
        chunk_key
    );

    let mut layer_meshes: HashMap<(YLevel, bool), HashMap<VoxelType, PosNormMesh>> = HashMap::new();
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
        if !full_detail {
            continue;
        }
        trace!(
            "Copying extent {:?} for layer {:?}",
            extent_to_copy,
            padded_layer_extent
        );
        let meshes = generate_mesh_for_extent_with_surface_nets(
            map_ref,
            chunk_key,
            &local_cache,
            padded_layer_extent,
            &extent_to_copy,
        );
        if let Some(meshes) = meshes {
            layer_meshes.insert(
                (
                    YLevel {
                        value: padded_layer_extent.max().y() - 1,
                    },
                    full_detail,
                ),
                meshes,
            );
        } else {
            // If the layer is empty, don't bother inserting the mesh (just discard it), and we can also
            // break out of the loop since if a bigger cut of the chunk was empty then any subsets of that
            // cut will also be empty.
            break;
        }
    }

    // If all the meshes of all the layers are empty, don't return anything.
    if layer_meshes.is_empty() {
        (*chunk_key, None)
    } else {
        (*chunk_key, Some(layer_meshes))
    }
}

fn generate_mesh_for_extent_with_greedy_quads(
    map_ref: &CompressibleChunkMap3<Voxel>,
    chunk_key: &Point3i,
    local_cache: &LocalChunkCache3<Voxel>,
    padded_extent: Extent3i,
    extent_to_copy: &Extent3i,
) -> Option<HashMap<VoxelType, PosNormMesh>> {
    trace!("Generating mesh for chunk at {:?}", chunk_key);
    let reader = map_ref.storage().reader(&local_cache);
    let reader_map: ChunkMap<
        [i32; 3],
        Voxel,
        (),
        CompressibleChunkStorageReader<[i32; 3], Voxel, (), Snappy>,
    > = DEFAULT_BUILDER.build_with_read_storage(reader);
    trace!(
        "Copying extent {:?} for padded extent {:?}",
        extent_to_copy,
        padded_extent
    );
    let mut padded_array = Array3::fill(padded_extent, EMPTY_VOXEL);
    copy_extent(&extent_to_copy, &reader_map, &mut padded_array);
    let lookup = |v: Voxel| *v.voxel_type();
    let voxel_types = TransformMap::new(&padded_array, lookup);

    // TODO bevy: we could avoid re-allocating the buffers on every call if we had
    // thread-local storage accessible from this task
    let mut buffer = GreedyQuadsBuffer::new(padded_extent);
    greedy_quads(&voxel_types, &padded_extent, &mut buffer);

    // Separate the meshes by material, so that we can render each voxel type with a different color.
    let mut meshes: HashMap<VoxelType, PosNormMesh> = HashMap::new();
    for group in buffer.quad_groups.iter() {
        for quad in group.quads.iter() {
            let material = *reader_map.get(&quad.minimum).voxel_type();
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
        None
    } else {
        Some(meshes)
    }
}

fn generate_mesh_for_extent_with_surface_nets(
    map_ref: &CompressibleChunkMap3<Voxel>,
    chunk_key: &Point3i,
    local_cache: &LocalChunkCache3<Voxel>,
    padded_extent: Extent3i,
    extent_to_copy: &Extent3i,
) -> Option<HashMap<VoxelType, PosNormMesh>> {
    trace!("Generating mesh for chunk at {:?}", chunk_key);
    let reader = map_ref.storage().reader(&local_cache);
    let reader_map: ChunkMap<
        [i32; 3],
        Voxel,
        (),
        CompressibleChunkStorageReader<[i32; 3], Voxel, (), Snappy>,
    > = DEFAULT_BUILDER.build_with_read_storage(reader);
    trace!(
        "Copying extent {:?} for padded extent {:?}",
        extent_to_copy,
        padded_extent
    );
    let mut padded_array = Array3::fill(padded_extent, EMPTY_VOXEL);
    copy_extent(&extent_to_copy, &reader_map, &mut padded_array);

    // TODO bevy: we could avoid re-allocating the buffers on every call if we had
    // thread-local storage accessible from this task
    let mut buffer = SurfaceNetsBuffer::default();
    surface_nets(&padded_array, &padded_extent, &mut buffer);

    if buffer.mesh.is_empty() {
        return None;
    }

    let SurfaceNetsBuffer {
        mesh,
        surface_strides,
        ..
    } = buffer;

    let lookup = |v: Voxel| *v.voxel_type();
    let voxel_types = TransformMap::new(&padded_array, lookup);
    let material_counts = count_adjacent_materials(&voxel_types, &surface_strides);

    // Separate the meshes by material, so that we can render each voxel type with a different color.
    let mut meshes: HashMap<VoxelType, PosNormMesh> = HashMap::new();
    // TODO: surface nets meshes don't have a material
    meshes.insert(VoxelType::Gold, mesh);

    Some(meshes)
}

pub trait TypedVoxel {
    fn voxel_type(&self) -> VoxelType;
}

impl TypedVoxel for VoxelType {
    fn voxel_type(&self) -> VoxelType {
        *self
    }
}

/// Uses a kernel to count the adjacent materials for each surface point. This is necessary because we used dual contouring to
/// construct the mesh, so a given vertex has 8 adjacent voxels, some of which may be empty. This also assumes that the material
/// layer can only be one of 0..NUM_VOXEL_TYPES.
fn count_adjacent_materials<A, V>(voxels: &A, surface_strides: &[Stride]) -> Vec<[u8; NUM_VOXEL_TYPES]>
where
    A: Array<[i32; 3]> + GetUncheckedRelease<Stride, V>,
    V: IsEmpty + TypedVoxel,
{
    let mut corner_offsets = [Stride(0); 8];
    voxels.strides_from_local_points(
        &Local::localize_points(&Point3i::corner_offsets()),
        &mut corner_offsets,
    );
    let mut material_counts = vec![[0; NUM_VOXEL_TYPES]; surface_strides.len()];
    for (stride, counts) in surface_strides.iter().zip(material_counts.iter_mut()) {
        for corner in corner_offsets.iter() {
            let corner_voxel = voxels.get(*stride + *corner);
            // Only add weights from non-empty voxels.
            if !corner_voxel.is_empty() {
                let material = corner_voxel.voxel_type();
                debug_assert!(material != VoxelType::Air);
                counts[material.index()] += 1;
            }
        }
    }

    material_counts
}

fn generate_mesh_entity(
    mesh: PosNormMesh,
    collidable: bool,
    commands: &mut Commands,
    material: (Handle<StandardMaterial>, HatMaterial),
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

    let commands = if full_detail {
        commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone_weak(),
                material: material.0,
                visible: Visible {
                    // Only the full-detail meshes of each chunk are visible to start with.
                    // Slices of chunks become visible as the player lowers the y-level.
                    is_visible: full_detail,
                    is_transparent,
                },
                ..pbr_bundle()
            })
            .with(Chunk)
            .with(y_level)
            .with(FullDetailMesh)
    } else {
        commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone_weak(),
                material: material.0,
                visible: Visible {
                    // Only the full-detail meshes of each chunk are visible to start with.
                    // Slices of chunks become visible as the player lowers the y-level.
                    is_visible: true,
                    is_transparent,
                },
                ..PbrBundle::default()
            })
            .with(Chunk)
            .with(y_level)
    };
    let entity = commands
        .current_entity()
        .expect("failed to get current entity");
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
