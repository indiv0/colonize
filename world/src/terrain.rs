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

    pub fn get_glyph(&self) -> char {
        match *self {
            Air => ' ',
            OutOfBounds => '?',
            Wall => 177u8 as char,
        }
    }

    /// Returns the glyph for a tile which is a level lower than the rendered
    /// level.
    pub fn get_lower_glyph(&self) -> char {
        match *self {
            Air => ' ',
            OutOfBounds => '?',
            Wall => '.',
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
