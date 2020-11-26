extern crate bevy;
extern crate bevy_mod_picking;
#[cfg(target_arch = "wasm32")]
extern crate bevy_webgl2;
extern crate building_blocks;
extern crate rand;

mod camera;
mod greeting;

use std::convert::TryInto;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::render::pipeline::PrimitiveTopology;
use bevy::tasks::{ComputeTaskPool, TaskPool};
use bevy_mod_picking::*;
use building_blocks::core::{Extent3i, PointN};
use building_blocks::mesh::{MaterialVoxel, PosNormMesh, GreedyQuadsBuffer, greedy_quads, padded_greedy_quads_chunk_extent};
use building_blocks::storage::{ChunkMap, ChunkMapReader, Array3, IsEmpty, Snappy};
use building_blocks::prelude::{copy_extent, LocalChunkCache3};
use rand::distributions::{Distribution, Uniform};

use camera::fps::{CameraState, CameraMovementPlugin};
use greeting::HelloPlugin;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let default_plugins = DefaultPlugins;
    #[cfg(target_arch = "wasm32")]
    let default_plugins = bevy_webgl2::DefaultPlugins;

    App::build()
        .add_plugins(default_plugins)
        .add_plugin(HelloPlugin)
        .add_plugin(CameraMovementPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(DebugPickingPlugin)
        .add_startup_system(setup.system())
        .add_system(toggle_cursor.system())
        .add_system(mesh_generator_system.system())
        .run();
}

// Setup a simple 3D scene.
fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add entities to the world.
    commands
        //// Plane
        //.spawn(PbrBundle {
        //    mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        //    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        //    ..Default::default()
        //})
        //// Cube
        //.spawn(PbrBundle {
        //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        //    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        //    transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        //    ..Default::default()
        //})
        //// Chunk
        //.spawn(PbrBundle {
        //    mesh: meshes.add(meshify_chunk(&Chunk::default())),
        //    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        //    transform: Transform::from_translation(Vec3::new(-5.0, -5.0, -5.0)),
        //    ..Default::default()
        //})
        //.with(PickableMesh::default())
        //.with(InteractableMesh::default())
        //.with(HighlightablePickMesh::default())
        //.with(SelectablePickMesh::default())
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, 10.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraState::default())
        .with(PickSource::default())
        // Mesh generation
        .insert_resource(MeshMaterial(materials.add(Color::rgb(1.0, 0.0, 0.0).into())))
        .insert_resource(MeshGeneratorState::new());
}

/// Toggles the cursor's visibility and lock mode when the space bar is pressed.
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
    }
}

const CHUNK_LEN: usize = 4096; // 16 * 16 * 16

struct Chunk([i8; CHUNK_LEN]);

impl Default for Chunk {
    fn default() -> Self {
        let terrain_density_range = Uniform::from(-34..16);
        let rng = rand::thread_rng();
        let boxed_array: Box<[i8; CHUNK_LEN]> = terrain_density_range.sample_iter(rng).take(CHUNK_LEN).collect::<Vec<_>>().into_boxed_slice().try_into().expect(&format!("expected a vec of length {}", CHUNK_LEN));
        Self(*boxed_array)
    }
}

struct MeshGeneratorState {
    mesh_entities: Vec<Entity>,
}

impl MeshGeneratorState {
    fn new() -> Self {
        Self { mesh_entities: Vec::new() }
    }
}

const CHUNK_SIZE: i32 = 16;

#[derive(Default)]
pub struct MeshMaterial(pub Handle<StandardMaterial>);

fn mesh_generator_system(commands: &mut Commands, pool: Res<ComputeTaskPool>, mut state: ResMut<MeshGeneratorState>, mut meshes: ResMut<Assets<Mesh>>, material: Res<MeshMaterial>) {
    if !state.mesh_entities.is_empty() { return }

    let chunk_meshes = generate_chunk_meshes_from_cubic(Cubic::Terrace, &pool.0);

    for mesh in chunk_meshes.into_iter() {
        if let Some(mesh) = mesh {
            state.mesh_entities.push(create_mesh_entity(mesh, commands, material.0.clone(), &mut meshes));
        }
    }
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cubic {
    Terrace,
}

impl Cubic {
    fn get_voxels(&self) -> Array3<CubeVoxel> {
        match self {
            Cubic::Terrace => {
                let extent =
                    Extent3i::from_min_and_shape(PointN([-20; 3]), PointN([40; 3])).padded(1);
                /*
                let mut voxels = Array3::fill(extent, CubeVoxel(false));
                for i in 0..40 {
                    let level = Extent3i::from_min_and_shape(
                        PointN([i - 20; 3]),
                        PointN([40 - i, 1, 40 - i]),
                    );
                    voxels.fill_extent(&level, CubeVoxel(true));
                }
                */
                let terrain_density_range = Uniform::from(-34..16);
                let rng = rand::thread_rng();
                let mut iter = terrain_density_range.sample_iter(rng);
                let voxels = Array3::fill_with(extent, |point| {
                    if iter.next().unwrap() > 0 {
                        CubeVoxel(true)
                    } else {
                        CubeVoxel(false)
                    }
                });

                voxels
            }
        }
    }
}

fn generate_chunk_meshes_from_cubic(cubic: Cubic, pool: &TaskPool) -> Vec<Option<PosNormMesh>> {
    let voxels = cubic.get_voxels();

    // Chunk up the voxels just to show that meshing across chunks is consistent.
    let chunk_shape = PointN([CHUNK_SIZE; 3]);
    let ambient_value = CubeVoxel(false);
    let default_chunk_meta = ();
    // Normally we'd keep this map around in a resource, but we don't need to for this specific
    // example. We could also use an Array3 here instead of a ChunkMap, but we use chunks for
    // educational purposes.
    let mut map = ChunkMap::new(
        chunk_shape,
        ambient_value,
        default_chunk_meta,
        Snappy,
    );
    copy_extent(voxels.extent(), &voxels, &mut map);

    // Generate the chunk meshes.
    let map_ref = &map;

    pool.scope(|s| {
        for chunk_key in map_ref.chunk_keys() {
            s.spawn(async move {
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
                    None
                } else {
                    Some(mesh)
                }
            })
        }
    })
}

fn create_mesh_entity(
    mesh: PosNormMesh,
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    let num_vertices = mesh.positions.len();

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
    render_mesh.set_indices(Some(Indices::U32(mesh.indices.into_iter().map(|i| i as u32).collect())));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(render_mesh),
            material,
            ..Default::default()
        })
        .current_entity()
        .unwrap()
}
