extern crate bevy;
extern crate bevy_mod_picking;
#[cfg(target_arch = "wasm32")]
extern crate bevy_webgl2;
extern crate building_blocks;
extern crate rand;

mod camera;
mod dwarf;
mod terrain;

use bevy::prelude::*;
use bevy_mod_picking::*;

use camera::fps::{CameraMovementPlugin, CameraState};
use dwarf::DwarfPlugin;
use terrain::TerrainPlugin;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let default_plugins = DefaultPlugins;
    #[cfg(target_arch = "wasm32")]
    let default_plugins = bevy_webgl2::DefaultPlugins;

    #[cfg(not(target_arch = "wasm32"))]
    {
        App::build()
            .add_plugins(default_plugins)
            .add_plugin(DwarfPlugin)
            .add_plugin(CameraMovementPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugPickingPlugin)
            .add_startup_system(setup.system())
            .add_system(toggle_cursor.system())
            .add_plugin(TerrainPlugin)
            .run();
    }
    #[cfg(target_arch = "wasm32")]
    {
        App::build()
            .add_plugins(default_plugins)
            .add_plugin(DwarfPlugin)
            .add_plugin(CameraMovementPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugPickingPlugin)
            .add_startup_system(setup.system())
            .add_plugin(TerrainPlugin)
            .run();
    }
}

// Setup a simple 3D scene.
fn setup(
    commands: &mut Commands,
) {
    // Add entities to the world.
    commands
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

#[cfg(not(target_arch = "wasm32"))]
/// Toggles the cursor's visibility and lock mode when the space bar is pressed.
fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Space) {
        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());
    }
}