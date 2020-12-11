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
    for name in &names {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(-30.0, -40.0, -30.0)),
                ..Default::default()
            })
            .with((Dwarf, Name(name.to_string())));
    }
}

pub(crate) struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_dwarves.system());
    }
}
