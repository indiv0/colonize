use cgmath::{EuclideanSpace, Point2};
use world::CHUNK_SIZE;

use command::Command;

const MOVEMENT_SPEED: i32 = 1;

pub enum CameraAction {
    Move(MoveCameraDirection),
}

pub enum MoveCameraDirection {
    Backward,
    Down,
    Forward,
    Left,
    Right,
    Up,
}

pub struct Camera {
    position: Point2<i32>,
    height: usize,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Point2::origin(),
            height: 0,
        }
    }

    pub fn get_position(&self) -> &Point2<i32> {
        &self.position
    }

    pub fn get_height(&self) -> &usize {
        &self.height
    }

    pub fn move_in_direction(&mut self, direction: &MoveCameraDirection) {
        use self::MoveCameraDirection::*;

        match *direction {
            Backward => self.position[1] += MOVEMENT_SPEED,
            Down => {
                match self.height {
                    x if x >= 1 => self.height -= 1,
                    _ => {}
                }
            },
            Forward => self.position[1] -= MOVEMENT_SPEED,
            Left => self.position[0] -= MOVEMENT_SPEED,
            Right => self.position[0] += MOVEMENT_SPEED,
            Up => {
                match self.height {
                    x if (x + 1) < CHUNK_SIZE => self.height += 1,
                    _ => {}
                }
            },
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}

pub fn new_move_camera_command<'a>(direction: &'a MoveCameraDirection, camera: &'a mut Camera) -> Command<'a> {
    Box::new(move || { camera.move_in_direction(direction) })
}
