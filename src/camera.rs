#[cfg(not(target_arch = "wasm32"))]
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub(crate) struct CameraMovement {
    /// Constant multiplier for camera translation speed.
    translation_speed: f32,
    /// Constant multiplier for camera rotation speed.
    rotation_speed: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for CameraMovement {
    fn default() -> Self {
        Self {
            translation_speed: 1.5,
            #[cfg(not(target_arch = "wasm32"))]
            rotation_speed: 10.0,
            #[cfg(target_arch = "wasm32")]
            rotation_speed: 20.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

pub(crate) struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_movement_system.system());
        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(mouse_movement_system.system());
    }
}

fn keyboard_movement_system(time: Res<Time>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut CameraMovement, &mut Transform)>) {
    for (camera, mut transform) in query.iter_mut() {
        let axis_backward = movement_direction(&keyboard_input, KeyCode::S, KeyCode::W);
        let axis_right = movement_direction(&keyboard_input, KeyCode::D, KeyCode::A);
        let axis_up = movement_direction(&keyboard_input, KeyCode::LShift, KeyCode::LControl);

        // Forward motion should occur along the "forward vector", which is computed by taking the
        // Z-axis unit vector and applying the camera's current rotation matrix to it.
        let forward_vector = transform.rotation.mul_vec3(Vec3::unit_z()).normalize();
        // Side-to-side motion should occur perpendicularly to the "forward vector", so we rotate
        // the "forward vector" 90 degrees to compute the "strafe vector".
        let strafe_vector = Quat::from_rotation_y(90.0f32.to_radians()).mul_vec3(forward_vector);
        // Compute the direction of motion by multiplying each axis vector against the signed
        // vectors, then add them together to compute an overall vector for the magnitude and
        // direction of motion.
        let mut velocity = (strafe_vector * axis_right + forward_vector * axis_backward + Vec3::unit_y() * axis_up).normalize();
        // Multiply the velocity vector by the speed constant so that the camera moves faster.
        velocity = velocity * camera.translation_speed * time.delta_seconds;
        // If we are not currently moving, one or more of the vector's components will be NaN so we
        // need to zero out the vector to avoid corrupting the translation matrix.
        if velocity.is_nan().all() { velocity = Vec3::zero() }

        transform.translation += velocity;
    }

    // When running in the browser, mouse movement events don't get reported for some reason so we use
    // the arrow keys to rotate the camera instead.
    #[cfg(target_arch = "wasm32")]
    {
        let axis_yaw = movement_direction(&keyboard_input, KeyCode::Right, KeyCode::Left);
        let axis_pitch = movement_direction(&keyboard_input, KeyCode::Up, KeyCode::Down);

        for (mut camera, mut transform) in query.iter_mut() {
            camera.yaw -= axis_yaw * camera.rotation_speed * time.delta_seconds;
            camera.pitch += axis_pitch * camera.rotation_speed * time.delta_seconds;
            transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), camera.yaw.to_radians()) * Quat::from_axis_angle(-Vec3::unit_x(), camera.pitch.to_radians());
        }
    }
}

fn movement_direction(input: &Res<Input<KeyCode>>, positive: KeyCode, negative: KeyCode) -> f32 {
    let mut direction = 0.0;
    if input.pressed(positive) {
        direction += 1.0;
    }
    if input.pressed(negative) {
        direction -= 1.0;
    }
    direction
}

#[cfg(not(target_arch = "wasm32"))]
fn mouse_movement_system(time: Res<Time>, mouse_motion_events: Res<Events<MouseMotion>>, mut query: Query<(&mut CameraMovement, &mut Transform)>) {
    let mut delta = Vec2::zero();
    for event in mouse_motion_events.get_reader().iter(&mouse_motion_events) {
        delta += event.delta;
    }

    for (mut camera, mut transform) in query.iter_mut() {
        camera.yaw -= delta.x * camera.rotation_speed * time.delta_seconds;
        camera.pitch += delta.y * camera.rotation_speed * time.delta_seconds;
        transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), camera.yaw.to_radians()) * Quat::from_axis_angle(-Vec3::unit_x(), camera.pitch.to_radians());
    }
}
