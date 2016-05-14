#[derive(Clone, Deserialize, Serialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}
