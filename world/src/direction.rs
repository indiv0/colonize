use cgmath::Vector3;

#[cfg(feature = "nightly")]
include!("direction.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/direction.rs"));

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
