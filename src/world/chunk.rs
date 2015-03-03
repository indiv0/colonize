use array::Array;

use terrain::{ Tile, TileType };
use world::CHUNK_SIZE;

fn array_16x16x16<T, F>(mut f: F) -> [[[T; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
    where F: FnMut(usize, usize, usize) -> T
{
    Array::from_fn(|y| -> [[T; CHUNK_SIZE]; CHUNK_SIZE]
        Array::from_fn(|z| -> [T; 16]
            Array::from_fn(|x| f(x, y, z))
        )
    )
}

pub struct Chunk {
    pub tiles: [[[Tile; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
}

impl Chunk {
    pub fn generate(height_map: [[f32; CHUNK_SIZE]; CHUNK_SIZE]) -> Chunk {
        Chunk {
            tiles: array_16x16x16(|x, y, z| {
                let height = (height_map[x][z] * CHUNK_SIZE as f32) as usize;
                Tile {
                    tile_type: match height {
                            h if h < y => TileType::Air,
                            _ => TileType::Wall,
                        }
                }
            }),
        }
    }
}
