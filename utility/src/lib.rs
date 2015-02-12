#[derive(Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Bounds {
        Bounds {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y },
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.min.x &&
        point.x < self.max.x &&
        point.y >= self.min.y &&
        point.y < self.max.y
    }

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }
}
