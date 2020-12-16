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

use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::render::pipeline::PrimitiveTopology;
use bevy::tasks::ComputeTaskPool;
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::Point};
use building_blocks::prelude::{copy_extent, LocalChunkCache3};
use building_blocks::storage::{Array3, ChunkMap, ChunkMapReader, IsEmpty, Snappy};
use building_blocks::{
    core::{Extent3i, Point3i, PointN},
    storage::ChunkMap3,
};
use building_blocks::{
    mesh::{
        greedy_quads, padded_greedy_quads_chunk_extent, GreedyQuadsBuffer, MaterialVoxel,
        PosNormMesh,
    },
    storage::{ForEachMut, LocalChunkCache},
};
use noise::{Fbm, MultiFractal, NoiseFn};

const CHUNK_SIZE: usize = 64;
const REGION_SIZE: usize = 128; // CHUNK_SIZE * NUM_CHUNKS

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TerrainResource::default())
            .add_resource(MeshResource::default())
            .add_startup_system(setup.system())
            .add_system(generate_voxels.system())
            .add_system(generate_meshes.system())
            .add_system(modify_config.system());
    }
}

fn setup(mut res: ResMut<TerrainResource>, mut materials: ResMut<Assets<StandardMaterial>>) {
    res.materials.push(MeshMaterial(
        materials.add(Color::rgb(0.376, 0.502, 0.22).into()), // Grass
    ));
}

pub(crate) struct TerrainResource {
    materials: Vec<MeshMaterial>,
    noise: Fbm,
    chunks: ChunkMap3<CubeVoxel>,
    generated_voxels: bool,
    sea_level: f64,
    y_offset: f64,
}

impl TerrainResource {
    fn y_scale(&self) -> f64 {
        self.sea_level * 0.2
    }

    /// Returns the y-value of the surface of the terrain at the specified x & z coordinates,
    /// as determined solely by the terrain generation algorithm.
    ///
    /// If the terrain gets modified after generation (e.g. digging holes or adding structures)
    /// the modifications are not taken into account by this function.
    pub(crate) fn surface_y(&self, x: f64, z: f64) -> i32 {
        // FIXME: I don't think this calculation is actually correct.
        (self.noise.get([x, z]) * self.y_scale() + self.y_offset).round() as i32
    }
}

impl Default for TerrainResource {
    fn default() -> Self {
        Self {
            materials: Vec::new(),
            noise: Fbm::new().set_frequency(0.008).set_octaves(5),
            chunks: ChunkMap::new(PointN([CHUNK_SIZE as i32; 3]), CubeVoxel(false), (), Snappy),
            generated_voxels: false,
            sea_level: 100.,
            y_offset: 10.,
        }
    }
}

struct MeshResource {
    meshes: HashMap<Point3i, Option<(Entity, Handle<Mesh>)>>,
}

impl Default for MeshResource {
    fn default() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct MeshMaterial(pub Handle<StandardMaterial>);

#[derive(Clone, Copy)]
struct CubeVoxel(bool);

impl MaterialVoxel for CubeVoxel {
    type Material = u8;

    fn material(&self) -> Self::Material {
        1 // only 1 material
    }
}

impl IsEmpty for CubeVoxel {
    fn is_empty(&self) -> bool {
        !self.0
    }
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

/// Removes all voxels & meshes and marks the world for regeneration.
fn reset_world(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    terrain_res: &mut ResMut<TerrainResource>,
    mesh_res: &mut ResMut<MeshResource>,
) {
    // Delete the voxels associated with the current world.
    terrain_res.chunks =
        ChunkMap::new(PointN([CHUNK_SIZE as i32; 3]), CubeVoxel(false), (), Snappy);

    // Delete the entities and meshes associated with the current world.
    let to_remove = mesh_res.meshes.keys().cloned().collect::<Vec<_>>();
    for p in to_remove {
        if let Some(Some((entity, mesh))) = mesh_res.meshes.remove(&p) {
            commands.despawn(entity);
            mesh_assets.remove(&mesh);
        }
    }

    // Mark the world as ungenerated.
    terrain_res.generated_voxels = false;
}

fn generate_voxels(mut terrain_res: ResMut<TerrainResource>) {
    if terrain_res.generated_voxels {
        return;
    }

    let min = PointN([-(REGION_SIZE as i32 / 2); 3]);
    let max = PointN([REGION_SIZE as i32 / 2; 3]);
    trace!("Generating voxels between {:?} and {:?}", min, max);

    let material_from_height = |height| {
        if height < 0.9 * terrain_res.y_scale() {
            CubeVoxel(true)
        } else {
            CubeVoxel(false)
        }
    };

    let local_cache = LocalChunkCache::new();
    let query_extent = Extent3i::from_min_and_shape(min, PointN([REGION_SIZE as i32; 3]));
    let mut dense_map = Array3::fill(query_extent, CubeVoxel(false));
    for z in min.z()..max.z() {
        for x in min.x()..max.x() {
            let max_y = (terrain_res.noise.get([x as f64, z as f64]) * terrain_res.y_scale()
                + terrain_res.y_offset)
                .round() as i32;
            let y_extent =
                Extent3i::from_min_and_shape(PointN([x, min.y(), z]), PointN([1, max_y + 1, 1]));
            dense_map.for_each_mut(&y_extent, |p: Point3i, value| {
                *value = material_from_height(p.y() as f64)
            });
        }
    }
    copy_extent(&query_extent, &dense_map, &mut terrain_res.chunks);
    // Ensure that we inserted the expected number of chunks.
    assert_eq!(
        terrain_res
            .chunks
            .chunk_keys()
            .collect::<Vec<&PointN<[i32; 3]>>>()
            .len(),
        (REGION_SIZE / CHUNK_SIZE).pow(3) as usize
    );
    terrain_res.chunks.flush_chunk_cache(local_cache);

    terrain_res.generated_voxels = true;
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
    for mesh in meshes.into_iter() {
        if let (p, Some(mesh)) = mesh {
            let entity = generate_mesh_entity(
                mesh,
                commands,
                terrain.materials[0].0.clone(),
                &mut mesh_assets,
            );
            trace!("Inserting {:?} into the mesh map", p);
            mesh_res.meshes.insert(p, Some(entity));
        } else if let (p, None) = mesh {
            // Insert points with no associated mesh into the hash map.
            // We use the presence of the chunk key in the hash map as a flag on
            // whether or not to generate the mesh. Chunks without meshes (i.e.
            // chunks with just air) shouldn't be regenerated so we add them to
            // the hash map as well.
            mesh_res.meshes.insert(p, None);
        }
    }
}

async fn generate_mesh(
    map_ref: &ChunkMap3<CubeVoxel>,
    chunk_key: &Point3i,
) -> (Point3i, Option<PosNormMesh>) {
    trace!("Generating mesh for chunk at {:?}", chunk_key);
    let local_cache = LocalChunkCache3::new();
    let map_reader = ChunkMapReader::new(map_ref, &local_cache);
    let padded_chunk_extent =
        padded_greedy_quads_chunk_extent(&map_ref.extent_for_chunk_at_key(chunk_key));

    let mut padded_chunk = Array3::fill(padded_chunk_extent, CubeVoxel(false));
    copy_extent(&padded_chunk_extent, &map_reader, &mut padded_chunk);

    // TODO bevy: we could avoid re-allocating the buffers on every call if we had
    // thread-local storage accessible from this task
    let mut buffer = GreedyQuadsBuffer::new(padded_chunk_extent);
    greedy_quads(&padded_chunk, &padded_chunk_extent, &mut buffer);

    let mut mesh = PosNormMesh::default();
    for group in buffer.quad_groups.iter() {
        for (quad, _material) in group.quads.iter() {
            group.face.add_quad_to_pos_norm_mesh(&quad, &mut mesh);
        }
    }

    if mesh.is_empty() {
        (chunk_key.clone(), None)
    } else {
        (chunk_key.clone(), Some(mesh))
    }
}

fn generate_mesh_entity(
    mesh: PosNormMesh,
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) -> (Entity, Handle<Mesh>) {
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    let num_vertices = mesh.positions.len();

    // Generate a colliding entity for the mesh to use for physics.
    // TODO: does this position actually need to match the position of the chunk mesh?
    //   It appears to work fine without any translation applied.
    let rigid_body = RigidBodyBuilder::new_static();
    let collider = ColliderBuilder::trimesh(
        nested_array_f32_to_points(&mesh.positions),
        flat_array_f32_to_points(&mesh.indices),
    );

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

    let entity = commands
        .spawn(PbrBundle {
            mesh: mesh_handle.clone_weak(),
            material,
            ..Default::default()
        })
        .with(rigid_body)
        .with(collider)
        .current_entity()
        .unwrap();
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
