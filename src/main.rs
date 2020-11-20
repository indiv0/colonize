extern crate bevy;
extern crate bevy_mod_picking;
#[cfg(target_arch = "wasm32")]
extern crate bevy_webgl2;
extern crate nalgebra;
extern crate rand;
extern crate tessellation;

mod camera;
mod greeting;

use std::convert::TryInto;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::PrimitiveTopology;
use bevy_mod_picking::*;
use nalgebra::{Point3, Vector3};
use rand::distributions::{Distribution, Uniform};
use tessellation::{BoundingBox, ImplicitFunction, ManifoldDualContouring/*, RealField*/};

use camera::fps::{CameraState, CameraMovementPlugin};
use greeting::HelloPlugin;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(HelloPlugin)
            .add_plugin(CameraMovementPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugPickingPlugin)
            .add_startup_system(setup.system())
            .add_system(toggle_cursor.system())
            .run();
    }
    #[cfg(target_arch = "wasm32")]
    {
        App::build()
            .add_plugins(bevy_webgl2::DefaultPlugins)
            .add_plugin(HelloPlugin)
            .add_plugin(CameraMovementPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugPickingPlugin)
            .add_startup_system(setup.system())
            .add_system(toggle_cursor.system())
            .run();
    }
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
        // Chunk
        .spawn(PbrBundle {
            mesh: meshes.add(meshify_chunk(&Chunk::default())),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(-5.0, -5.0, -5.0)),
            ..Default::default()
        })
        .with(PickableMesh::default())
        .with(InteractableMesh::default())
        .with(HighlightablePickMesh::default())
        .with(SelectablePickMesh::default())
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
        .with(PickSource::default());
}

/// Toggles the cursor's visibility and lock mode when the space bar is pressed.
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
    }
}

fn vec3_sub([ax, ay, az]: [f32; 3], [bx, by, bz]: [f32; 3]) -> [f32; 3] {
    [ax - bx, ay - by, az - bz]
}

fn vec3_cross([ax, ay, az]: [f32; 3], [bx, by, bz]: [f32; 3]) -> [f32; 3] {
    [ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx]
}

fn vec3_add([ax, ay, az]: [f32; 3], [bx, by, bz]: [f32; 3]) -> [f32; 3] {
    [ax + bx, ay + by, az + bz]
}

fn vec3_normalize([x, y, z]: [f32; 3]) -> [f32; 3] {
    let m = f32::sqrt(x * x  + y * y + z * z); // magnitude
    [x / m, y / m, z / m]
}

pub(crate) fn meshify_chunk(_chunk: &Chunk) -> Mesh {
    let sphere = UnitSphere::new();
    let mut mdc = ManifoldDualContouring::new(&sphere, 0.2, 0.1);
    let triangles = mdc.tessellate().unwrap();

    let indices = Indices::U32(triangles.faces.iter().flatten().map(|i| *i as u32).collect::<Vec<u32>>());
    let mut normals: Vec<[f32; 3]> = vec![[0.0; 3]; triangles.vertices.len()];
    for i in 0..triangles.faces.len() {
        let ia = triangles.faces[i][0];
        let ib = triangles.faces[i][1];
        let ic = triangles.faces[i][2];
 
        let e1 = vec3_sub(triangles.vertex32(ia), triangles.vertex32(ib));
        let e2 = vec3_sub(triangles.vertex32(ic), triangles.vertex32(ib));
        let no = vec3_cross(e1, e2);

        normals[ia] = vec3_add(normals[ia], no);
        normals[ib] = vec3_add(normals[ib], no);
        normals[ic] = vec3_add(normals[ic], no);
    }
    normals.iter_mut().for_each(|n| *n = vec3_normalize(*n));
    let positions = triangles.vertices;
    // Generate some zeroed-out (i.e. not correct) UVs to resolve an issue
    // where the WASM doesn't load on Chrome but works on Firefox.
    let uvs = vec![[0.; 2]; positions.len()];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    assert_eq!(positions.len(), normals.len(), "Must have same vertex count ({}) as normal count ({}) in this mesh", positions.len(), normals.len());
    assert_eq!(positions.len(), uvs.len(), "Must have same vertex count ({}) as uv count ({}) in this mesh", positions.len(), uvs.len());
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
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

struct UnitSphere {
    bbox: BoundingBox<f32>,
}

impl UnitSphere {
    fn new() -> Self {
        Self { bbox: BoundingBox::new(&Point3::new(-10., -10., -10.), &Point3::new(10., 10., 10.)) }
    }
}

impl ImplicitFunction<f32> for UnitSphere {
    fn bbox(&self) -> &BoundingBox<f32> {
        &self.bbox
    }

    fn value(&self, p: &Point3<f32>) -> f32 {
        Vector3::new(p.x, p.y, p.z).norm() - 10.0
    }

    fn normal(&self, p: &Point3<f32>) -> Vector3<f32> {
        Vector3::new(p.x, p.y, p.z).normalize()
    }
}
