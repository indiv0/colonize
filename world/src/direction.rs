use cgmath::Vector3;

pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn to_vector(&self) -> Vector3<i32> {
        use self::Direction::*;

        match *self {
            North => -Vector3::unit_z(),
            South => Vector3::unit_z(),
            West => -Vector3::unit_x(),
            East => Vector3::unit_x(),
            Up => Vector3::unit_y(),
            Down => -Vector3::unit_y(),
        }
    }
}
