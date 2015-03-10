use array::Array;

use CHUNK_SIZE;
use terrain::{ Tile, TileType };

pub type ChunkArray<T> = [T; CHUNK_SIZE];
pub type ChunkArray2d<T> = ChunkArray<ChunkArray<T>>;
pub type ChunkArray3d<T> = ChunkArray<ChunkArray2d<T>>;
pub type Tiles = ChunkArray3d<Tile>;

fn array_16x16x16<T, F>(mut f: F) -> ChunkArray3d<T>
    where F: FnMut(usize, usize, usize) -> T
{
    Array::from_fn(|y| -> ChunkArray2d<T> {
        Array::from_fn(|z| -> ChunkArray<T> {
            Array::from_fn(|x| f(x, y, z))
        })
    })
}

pub struct Chunk {
    pub tiles: Tiles,
}

impl Chunk {
    pub fn generate(height_map: ChunkArray2d<f64>) -> Chunk {
        Chunk {
            tiles: array_16x16x16(|x, y, z| {
                // TODO: fix this multiplier here for when I implement 3D chunk
                // generation.
                let height = (height_map[x][z] * CHUNK_SIZE as f64) as usize;
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
