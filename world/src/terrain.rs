use self::TileType::*;

// TODO: refactor these values to be configurable.
const WATER_LINE: i32 = 14;
const SOIL_DEPTH: i32 = 3;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum TileType {
    Air,
    Grass,
    OutOfBounds,
    Sand,
    Soil,
    Wall,
    Water,
}

impl TileType {
    pub fn is_solid(&self) -> bool {
        match *self {
            Grass | Sand | Soil | Wall | Water => true,
            Air | OutOfBounds => false,
        }
    }

    /// Returns the TileType for a tile at a specific elevation, provided the
    /// height_map specifies a `height` at this location.
    pub fn get_from_elevation(elevation: i32, height: i32) -> Self {
        match elevation {
            _ if elevation > height => {
                match elevation {
                    _ if elevation > WATER_LINE => TileType::Air,
                    _ => TileType::Water,
                }
            }
            _ => match elevation {
                _ if elevation > WATER_LINE => {
                    match elevation {
                        _ if elevation > height - 1 => TileType::Grass,
                        _ if elevation > height - SOIL_DEPTH => TileType::Soil,
                        _ => TileType::Wall,
                    }
                },
                _ => {
                    match elevation {
                        _ if elevation > height - SOIL_DEPTH => TileType::Sand,
                        _ => TileType::Wall,
                    }
                }
            }
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
