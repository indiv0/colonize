use bevy::{
    ecs::ResMut,
    math::Vec3,
    pbr::PbrBundle,
    prelude::Assets,
    prelude::{
        shape, AppBuilder, Color, Commands, IntoSystem, Mesh, Plugin, StandardMaterial, Transform,
    },
};

struct Dwarf;
struct Name(String);

fn add_dwarves(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    let mut x = -30.0;
    let mut z = -30.0;
    for name in &names {
        // Dynamic rigid-body with cuboid shape as the collision box.
        let rigid_body = RigidBodyBuilder::new_dynamic().translation(x, -40.0, z);
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(x, -40.0, z)),
                ..Default::default()
            })
            .with(Dwarf)
            .with(Name(name.to_string()))
            .with(rigid_body)
            .with(collider);

        // Space out the dwarves from each other.
        x -= 2.;
        z -= 2.;
    }
}

pub(crate) struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_dwarves.system());
    }
}
