use cgmath::{ElementWise, EuclideanSpace, Point3, Vector3};

use command::Command;
use world::Direction;

/// The speed at which the camera moves along the three axes.
const MOVEMENT_SPEED: Vector3<i32> = Vector3 { x: 1, y: 1, z: 1 };

pub enum CameraAction {
    Move(Direction),
}

pub struct Camera {
    position: Point3<i32>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Point3::origin(),
        }
    }

    pub fn get_position(&self) -> &Point3<i32> {
        &self.position
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        self.position += direction.to_vector().mul_element_wise(MOVEMENT_SPEED);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}

pub fn new_move_camera_command<'a>(direction: &'a Direction, camera: &'a mut Camera) -> Command<'a> {
    Box::new(move || { camera.move_in_direction(direction) })
}
