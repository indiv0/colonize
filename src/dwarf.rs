use bevy::{
    ecs::{Res, ResMut},
    input::Input,
    math::Vec3,
    pbr::PbrBundle,
    prelude::Assets,
    prelude::{
        shape, AppBuilder, Color, Commands, IntoSystem, KeyCode, Mesh, Plugin, StandardMaterial,
        Transform,
    },
};
use bevy_mod_picking::{HighlightablePickMesh, InteractableMesh, PickableMesh, SelectablePickMesh};
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use rand::{thread_rng, Rng};

use crate::terrain::TerrainResource;

struct Dwarf;
#[derive(Debug)]
struct Name(String);

fn add_dwarves(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_res: Res<TerrainResource>,
) {
    let names = [
        "Khustrul Lavablade",
        "Vaddul Cavemantle",
        "Ormat Caskfury",
        "Alfodgror Steelbeard",
        "Krorhunri Mithrilshoulder",
        "Noramilin Blazingsunder",
        "Snastaenelyn Whitbow",
        "Kizneabela Horndelver",
        "Korkkalynn Opalback",
        "Thulgreline Oakview",
    ];

    // Pick random starting positions for each dwarf, on the surface of the terrain, within
    // 30 blocks of origin. Ensure that none of the locations collide.
    let mut spawn_positions = Vec::new();
    while spawn_positions.len() < names.len() {
        let p = random_point_in_circle((0., 0.), 10.);
        // To ensure that dwarves don't spawn inside of each other (which would cause
        // physics problems) we check if the given X & Z coords are already in the list.
        if !spawn_positions.contains(&p) {
            spawn_positions.push(p);
        }
    }
    let mut spawn_positions = spawn_positions
        .into_iter()
        // FIXME: `surface_y` doesn't return the correct value. It returns a value that's way
        //   above the actual surface.
        .map(|(x, z)| (x as f32, terrain_res.surface_y(x, z) as f32, z as f32));

    for name in &names {
        let position = spawn_positions.next().unwrap();
        spawn_dwarf(
            name.to_string(),
            position,
            commands,
            &mut meshes,
            &mut materials,
        );
    }
}

fn spawn_dwarf(
    name: String,
    (px, py, pz): (f32, f32, f32),
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    const SIZE: f32 = 1.;
    // Dynamic rigid-body with cuboid shape as the collision box.
    let rigid_body = RigidBodyBuilder::new_dynamic().translation(px, py, pz);
    let collider = ColliderBuilder::cuboid(SIZE, SIZE, SIZE);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: SIZE })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(px, py, pz)),
            ..Default::default()
        })
        .with(Dwarf)
        .with(Name(name.to_string()))
        .with(rigid_body)
        .with(collider)
        .with(PickableMesh::default())
        .with(InteractableMesh::default())
        .with(HighlightablePickMesh::default())
        .with(SelectablePickMesh::default());
}

/// Chooses a random point with a circle.
fn random_point_in_circle(origin: (f64, f64), radius: f64) -> (f64, f64) {
    let mut rng = thread_rng();
    let r = radius * rng.gen::<f64>().sqrt();
    let theta = rng.gen::<f64>() * 2. * std::f64::consts::PI;
    let x = origin.0 + r * theta.cos();
    let y = origin.1 + r * theta.sin();
    (x, y)
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    terrain_res: Res<TerrainResource>,
) {
    // If the `T` button is pressed, spawn in 10 more dwarves.
    if keyboard_input.pressed(KeyCode::T) {
        add_dwarves(commands, meshes, materials, terrain_res);
    }
}

pub(crate) struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_dwarves.system())
            .add_system(input_system)
            .add_system(print_events);
    }
}
