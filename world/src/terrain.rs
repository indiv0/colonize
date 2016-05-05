use self::TileType::*;

#[derive(Clone, Copy)]
pub enum TileType {
    Air,
    OutOfBounds,
    Wall,
}

impl TileType {
    pub fn is_solid(&self) -> bool {
        match *self {
            Wall => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type: tile_type
        }
    }
}
