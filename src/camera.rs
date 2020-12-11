use bevy::prelude::*;

/// RTS-style camera implementation.
pub(crate) mod rts {
    #[cfg(not(target_arch = "wasm32"))]
    use bevy::input::mouse::MouseMotion;
    use bevy::input::mouse::MouseWheel;
    use bevy::prelude::*;

    use super::*;

    pub(crate) struct CameraState {
        focus: Vec3,
        rotation_speed: f32,
    }

    impl Default for CameraState {
        fn default() -> Self {
            Self {
                focus: Vec3::default(),
                #[cfg(not(target_arch = "wasm32"))]
                rotation_speed: 1.0,
                #[cfg(target_arch = "wasm32")]
                rotation_speed: 2.0,
            }
        }
    }

    pub(crate) struct CameraMovementPlugin;

    impl Plugin for CameraMovementPlugin {
        fn build(&self, app: &mut AppBuilder) {
            app.add_resource(InputState::default())
                .add_system(pan_orbit_camera.system());
        }
    }

    /// Hold readers for events.
    #[derive(Default)]
    struct InputState {
        #[cfg(not(target_arch = "wasm32"))]
        mouse_motion_event_reader: EventReader<MouseMotion>,
        mouse_scroll_event_reader: EventReader<MouseWheel>,
    }

    /// Pan the camera with LHold or scroll wheel, orbit with right click.
    #[cfg(not(target_arch = "wasm32"))]
    fn pan_orbit_camera(
        time: Res<Time>,
        windows: Res<Windows>,
        mut state: ResMut<InputState>,
        mouse_motion_events: Res<Events<MouseMotion>>,
        mouse_button_input: Res<Input<MouseButton>>,
        mouse_scroll_events: Res<Events<MouseWheel>>,
        query: Query<(&mut CameraState, &mut Transform)>,
    ) {
        let mut translation = Vec2::zero();
        let mut rotation_move = Vec2::default();

        if mouse_button_input.pressed(MouseButton::Right) {
            for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
                rotation_move += event.delta;
            }
        } else if mouse_button_input.pressed(MouseButton::Left) {
            // Pan only if we're not rotating at the moment.
            for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
                translation += event.delta;
            }
        }

        pan_orbit_camera_inner(
            &translation,
            &rotation_move,
            time,
            windows,
            state,
            mouse_scroll_events,
            query,
        );
    }

    /// Pan the camera with WASD or scroll wheel, orbit with arrow keys.
    // On wasm32 certain mouse events don't seem to work so we use WASD and arrow keys instead.
    #[cfg(target_arch = "wasm32")]
    fn pan_orbit_camera(
        time: Res<Time>,
        windows: Res<Windows>,
        state: ResMut<InputState>,
        keyboard_input: Res<Input<KeyCode>>,
        mouse_scroll_events: Res<Events<MouseWheel>>,
        query: Query<(&mut CameraState, &mut Transform)>,
    ) {
        let mut translation = Vec2::zero();
        let mut rotation_move = Vec2::default();

        let axis_horizontal = movement_direction(
            &keyboard_input,
            &[KeyCode::Down, KeyCode::Numpad2],
            &[KeyCode::Up, KeyCode::Numpad8],
        );
        let axis_vertical = movement_direction(
            &keyboard_input,
            &[KeyCode::Left, KeyCode::Numpad4],
            &[KeyCode::Right, KeyCode::Numpad6],
        );

        if axis_horizontal != 0.0 || axis_vertical != 0.0 {
            rotation_move.x += axis_vertical;
            rotation_move.y += axis_horizontal;
        } else {
            let axis_forward = movement_direction(&keyboard_input, &[KeyCode::W], &[KeyCode::S]);
            let axis_left = movement_direction(&keyboard_input, &[KeyCode::A], &[KeyCode::D]);

            translation.x += axis_left;
            translation.y += axis_forward;
        }

        pan_orbit_camera_inner(
            &translation,
            &rotation_move,
            time,
            windows,
            state,
            mouse_scroll_events,
            query,
        );
    }

    fn pan_orbit_camera_inner(
        translation: &Vec2,
        rotation_move: &Vec2,
        time: Res<Time>,
        windows: Res<Windows>,
        mut state: ResMut<InputState>,
        mouse_scroll_events: Res<Events<MouseWheel>>,
        mut query: Query<(&mut CameraState, &mut Transform)>,
    ) {
        let mut scroll = 0.0;

        for event in state.mouse_scroll_event_reader.iter(&mouse_scroll_events) {
            scroll += event.y;
        }

        // Either pan+scroll or arcball. Don't do both at once.
        for (mut camera, mut transform) in query.iter_mut() {
            if rotation_move.length_squared() > 0.0 {
                let window = windows.get_primary().unwrap();
                let window_w = window.width() as f32;
                let window_h = window.height() as f32;

                // Link virtual sphere rotation relative to window to make it feel nicer.
                let delta_x = (rotation_move.x * camera.rotation_speed) / window_w
                    * std::f32::consts::PI
                    * 2.0;
                let delta_y =
                    (rotation_move.y * camera.rotation_speed) / window_h * std::f32::consts::PI;

                let delta_yaw = Quat::from_rotation_z(delta_x);
                let delta_pitch = Quat::from_rotation_x(delta_y);
                let delta_rotation = delta_yaw.mul_quat(delta_pitch);
                // x y = pitch
                // z x = yaw
                //let delta_rotation = Quat::from_rotation_ypr(0.0, delta_y, delta_x);

                //transform.translation = delta_yaw * delta_pitch * (transform.translation - camera.focus) + camera.focus;
                transform.translation =
                    delta_rotation * (transform.translation - camera.focus) + camera.focus;

                let look = Mat4::face_toward(
                    transform.translation,
                    camera.focus,
                    Vec3::new(0.0, 0.0, 1.0),
                );
                transform.rotation = look.to_scale_rotation_translation().1;
            } else {
                let forward_vector = forward_vector(&transform.rotation);
                let strafe_vector = strafe_vector(forward_vector);
                let mut translation = (strafe_vector * -translation.x
                    + forward_vector * -translation.y)
                    * time.delta_seconds();
                translation.z = -scroll;
                camera.focus += translation;
                transform.translation += translation;
            }
        }
    }

    /// Side-to-side motion should occur perpendicularly to the "forward vector", so we rotate
    /// the "forward vector" 90 degrees to compute the "strafe vector".
    fn strafe_vector(forward_vector: Vec3) -> Vec3 {
        Quat::from_rotation_z(90.0f32.to_radians()).mul_vec3(forward_vector)
    }
}

/// FPS-style camera implementation.
pub(crate) mod fps {
    #[cfg(not(target_arch = "wasm32"))]
    use bevy::input::mouse::MouseMotion;
    use bevy::prelude::*;

    use super::*;

    pub(crate) struct CameraState {
        /// Constant multiplier for camera translation speed.
        translation_speed: f32,
        /// Constant multiplier for camera rotation speed.
        rotation_speed: f32,
        yaw: f32,
        pitch: f32,
    }

    impl Default for CameraState {
        fn default() -> Self {
            Self {
                translation_speed: 30.0,
                #[cfg(not(target_arch = "wasm32"))]
                rotation_speed: 10.0,
                #[cfg(target_arch = "wasm32")]
                rotation_speed: 20.0,
                yaw: 35.0,
                pitch: 45.0,
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

    fn keyboard_movement_system(
        time: Res<Time>,
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut CameraState, &mut Transform)>,
    ) {
        for (camera, mut transform) in query.iter_mut() {
            let axis_backward = movement_direction(&keyboard_input, &[KeyCode::S], &[KeyCode::W]);
            let axis_right = movement_direction(&keyboard_input, &[KeyCode::D], &[KeyCode::A]);
            let axis_up = movement_direction(&keyboard_input, &[KeyCode::Q], &[KeyCode::E]);

            let forward_vector = forward_vector(&transform.rotation).normalize();
            let strafe_vector =
                strafe_vector(Vec3::new(forward_vector.x, 0.0, forward_vector.z).normalize());
            // Compute the direction of motion by multiplying each axis vector against the signed
            // vectors, then add them together to compute an overall vector for the magnitude and
            // direction of motion.
            let mut velocity = (strafe_vector * axis_right
                + forward_vector * axis_backward
                + Vec3::unit_y() * axis_up)
                .normalize();
            // Multiply the velocity vector by the speed constant so that the camera moves faster.
            velocity = velocity * camera.translation_speed * time.delta_seconds();
            // If we are not currently moving, one or more of the vector's components will be NaN so we
            // need to zero out the vector to avoid corrupting the translation matrix.
            if velocity.is_nan() {
                velocity = Vec3::zero()
            }

            transform.translation += velocity;
        }

        // When running in the browser, mouse movement events don't get reported for some reason so we use
        // the arrow keys to rotate the camera instead.
        #[cfg(target_arch = "wasm32")]
        {
            let axis_yaw = movement_direction(
                &keyboard_input,
                &[KeyCode::Right, KeyCode::Numpad6],
                &[KeyCode::Left, KeyCode::Numpad4],
            );
            let axis_pitch = movement_direction(
                &keyboard_input,
                &[KeyCode::Up, KeyCode::Numpad8],
                &[KeyCode::Down, KeyCode::Numpad2],
            );

            for (mut camera, mut transform) in query.iter_mut() {
                camera.yaw -= axis_yaw * camera.rotation_speed * time.delta_seconds();
                camera.pitch += axis_pitch * camera.rotation_speed * time.delta_seconds();
                transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), camera.yaw.to_radians())
                    * Quat::from_axis_angle(-Vec3::unit_x(), camera.pitch.to_radians());
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn mouse_movement_system(
        time: Res<Time>,
        mouse_motion_events: Res<Events<MouseMotion>>,
        mut query: Query<(&mut CameraState, &mut Transform)>,
    ) {
        let mut delta = Vec2::zero();
        for event in mouse_motion_events.get_reader().iter(&mouse_motion_events) {
            delta += event.delta;
        }

        for (mut camera, mut transform) in query.iter_mut() {
            camera.yaw -= delta.x * camera.rotation_speed * time.delta_seconds();
            camera.pitch += delta.y * camera.rotation_speed * time.delta_seconds();
            transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), camera.yaw.to_radians())
                * Quat::from_axis_angle(-Vec3::unit_x(), camera.pitch.to_radians());
        }
    }

    /// Side-to-side motion should occur perpendicularly to the "forward vector", so we rotate
    /// the "forward vector" 90 degrees to compute the "strafe vector".
    fn strafe_vector(forward_vector: Vec3) -> Vec3 {
        Quat::from_rotation_y(90.0f32.to_radians())
            .mul_vec3(forward_vector)
            .normalize()
    }
}

fn movement_direction(
    input: &Res<Input<KeyCode>>,
    positive: &[KeyCode],
    negative: &[KeyCode],
) -> f32 {
    let mut direction = 0.0;
    if positive.iter().any(|k| input.pressed(*k)) {
        direction += 1.0;
    }
    if negative.iter().any(|k| input.pressed(*k)) {
        direction -= 1.0;
    }
    direction
}

/// Forward motion should occur along the "forward vector", which is computed by taking the
/// Z-axis unit vector and applying the camera's current rotation matrix to it.
fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::unit_z())
}
