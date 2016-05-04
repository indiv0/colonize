use array::Array;
use cgmath::Point3;

use {CHUNK_SIZE, HEIGHT_MAP_MULTIPLIER};
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
    pub fn generate(pos: Point3<i32>, height_map: ChunkArray2d<f64>) -> Chunk {
        let chunk_y = (pos.y * CHUNK_SIZE as i32) as f64;

        Chunk {
            tiles: array_16x16x16(|x, y, z| {
                let map_height = height_map[x][z] * HEIGHT_MAP_MULTIPLIER;
                let tile_y = chunk_y + y as f64;
                Tile {
                    tile_type: match map_height {
                            h if h < tile_y => TileType::Air,
                            _ => TileType::Wall,
                        }
                }
            }),
        }
    }
}
