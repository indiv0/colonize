use cgmath::{ElementWise, EuclideanSpace, Point3, Vector3};
use rgframework::Command;
use world::Direction;

#[cfg(feature = "nightly")]
include!("camera.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/camera.rs"));

pub struct Camera {
    /// The speed at which the camera moves along the three axes.
    movement_speed: Vector3<i32>,
    position: Point3<i32>,
}

impl Camera {
    pub fn new(movement_speed: Vector3<i32>, position: Point3<i32>) -> Self {
        Camera {
            movement_speed: movement_speed,
            position: position,
        }
    }

    pub fn get_position(&self) -> &Point3<i32> {
        &self.position
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        self.position += direction.to_vector().mul_element_wise(self.movement_speed);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            movement_speed: Vector3::new(1, 1, 1),
            position: Point3::origin(),
        }
    }
}

pub fn new_move_camera_command<'a>(direction: &'a Direction, camera: &'a mut Camera) -> Command<'a> {
    Box::new(move || { camera.move_in_direction(direction) })
}
