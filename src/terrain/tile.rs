#[derive(Copy)]
pub enum TileType {
    Air,
    Wall,
    OutOfBounds
}

#[derive(Copy)]
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
