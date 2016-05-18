use std::rc::Rc;

use cgmath::{
    ElementWise,
    Matrix4,
    Point3,
    Vector3,
    VectorSpace,
};
use glium::backend::Facade;
use rgframework::Command;
use world::Direction;

#[cfg(feature = "nightly")]
include!("camera.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/camera.rs"));

// Camera
// FIXME: move the initial position assignment to the game initializer
const INITIAL_POSITION: Point3<i32> = Point3 { x: 0, y: 15, z: 1};
const MOVEMENT_SPEED: Vector3<i32> = Vector3 { x: 1, y: 1, z: 1 };

pub struct Camera<F>
    where F: Facade,
{
    facade: Rc<F>,
    /// The speed at which the camera moves along the three axes.
    movement_speed: Vector3<i32>,
    position: Point3<i32>,
    offset: Vector3<f32>,
}

impl<F> Camera<F>
    where F: Facade,
{
    pub fn new(facade: Rc<F>) -> Self {
        Camera {
            facade: facade,
            movement_speed: MOVEMENT_SPEED,
            position: INITIAL_POSITION,
            offset: Vector3::zero(),
        }
    }

    pub fn get_position(&self) -> &Point3<i32> {
        &self.position
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        self.position += direction.to_vector().mul_element_wise(self.movement_speed);
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        // FIXME: retrieve this using the hidpi_factor() method of `Display`.
        let factor = 1.0;
        let (w, h) = self.facade.get_context().get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Matrix4::new(
            f / w, 0.0, 0.0, 0.0,
            0.0, f / h, 0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ) * Matrix4::from_translation(-self.offset)
    }
}

pub fn new_move_camera_command<'a, F>(direction: &'a Direction, camera: &'a mut Camera<F>) -> Command<'a>
    where F: Facade,
{
    Box::new(move || { camera.move_in_direction(direction) })
}
