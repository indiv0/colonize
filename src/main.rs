extern crate bevy;
extern crate bevy_mod_picking;
extern crate bevy_rapier3d;
#[cfg(target_arch = "wasm32")]
extern crate bevy_webgl2;
extern crate building_blocks;
extern crate colonize_common;
extern crate rand;

mod camera;
mod dwarf;
mod terrain;

use bevy::{
    app::startup_stage,
    ecs::{Commands, IntoSystem, Res, ResMut, SystemStage},
    input::Input,
    math::Vec3,
    prelude::{App, Camera3dBundle, KeyCode, Transform},
    window::Windows,
    DefaultPlugins,
};
use bevy_mod_picking::{DebugPickingPlugin, InteractablePickingPlugin, PickSource, PickingPlugin};
use bevy_rapier3d::physics::RapierPhysicsPlugin;

use camera::fps::{CameraMovementPlugin, CameraState};
use colonize_pbr::{LightBundle, PbrPlugin};
use dwarf::{DwarfPlugin, DWARVES};
use terrain::{TerrainPlugin, TERRAIN};

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let default_plugins = DefaultPlugins;
    #[cfg(target_arch = "wasm32")]
    let default_plugins = bevy_webgl2::DefaultPlugins;

    #[cfg(not(target_arch = "wasm32"))]
    {
        App::build()
            .add_startup_stage_after(startup_stage::PRE_STARTUP, TERRAIN, SystemStage::parallel())
            .add_startup_stage_after(TERRAIN, DWARVES, SystemStage::parallel())
            .add_plugins(default_plugins)
            .add_plugin(DwarfPlugin)
            .add_plugin(CameraMovementPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugPickingPlugin)
            .add_startup_system(setup.system())
            .add_system(toggle_cursor.system())
            .add_plugin(TerrainPlugin)
            .add_plugin(RapierPhysicsPlugin)
            .add_plugin(PbrPlugin)
            .run();
    }
    #[cfg(target_arch = "wasm32")]
    {
        App::build()
            .add_startup_stage_after(startup_stage::STARTUP, TERRAIN, SystemStage::parallel())
            .add_startup_stage_after(TERRAIN, DWARVES, SystemStage::parallel())
            .add_plugins(default_plugins)
            .add_plugin(DwarfPlugin)
            .add_plugin(CameraMovementPlugin)
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(DebugPickingPlugin)
            .add_startup_system(setup.system())
            .add_plugin(TerrainPlugin)
            .add_plugin(RapierPhysicsPlugin)
            .add_plugin(PbrPlugin)
            .run();
    }
}

// Setup a simple 3D scene.
fn setup(commands: &mut Commands) {
    // Add entities to the world.
    commands
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 200.0, 4.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(32.0, 100.0, 32.0))
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
