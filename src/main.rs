extern crate bevy;
#[cfg(target_arch = "wasm32")]
extern crate bevy_webgl2;

use bevy::prelude::*;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(HelloPlugin)
            .add_startup_system(setup.system())
            .run();
    }
    #[cfg(target_arch = "wasm32")]
    {
        App::build()
            .add_plugins(bevy_webgl2::DefaultPlugins)
            .add_plugin(HelloPlugin)
            .add_startup_system(setup.system())
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
        // Plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // Cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

struct Person;
struct Name(String);

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    // Update our timer with the time elapsed since the last update.
    timer.0.tick(time.delta_seconds);

    // Check to see if the timer has finished. If it has, print out message.
    if timer.0.finished {
        for (_person, name) in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub(crate) struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}
