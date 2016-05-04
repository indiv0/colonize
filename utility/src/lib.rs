#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cgmath;

use std::ops::Sub;

use cgmath::{BaseNum, Point2};

#[derive(Copy, Clone)]
pub struct Bounds<T>
    where T: Copy + Clone
{
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T> Bounds<T>
    where T: BaseNum + Copy + Clone + PartialOrd + Sub<Output=T>
{
    pub fn new(min_x: T, min_y: T, max_x: T, max_y: T) -> Bounds<T> {
        Bounds {
            min: Point2::new(min_x, min_y),
            max: Point2::new(max_x, max_y),
        }
    }

    pub fn contains(&self, point: Point2<T>) -> bool {
        point[0] >= self.min[0] &&
        point[0] < self.max[0] &&
        point[1] >= self.min[1] &&
        point[1] < self.max[1]
    }

    pub fn width(&self) -> T {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> T {
        self.max[1] - self.min[1]
    }
}
