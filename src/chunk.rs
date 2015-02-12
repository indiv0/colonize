use gfx_voxel::array::Array;

use terrain::tile::{ Tile, TileType };

pub const SIZE: usize = 16;

fn array_16x16x16<T, F>(mut f: F) -> [[[T; SIZE]; SIZE]; SIZE]
    where F: FnMut(usize, usize, usize) -> T
{
    Array::from_fn(|y| -> [[T; SIZE]; SIZE]
        Array::from_fn(|z| -> [T; 16]
            Array::from_fn(|x| f(x, y, z))
        )
    )
}

pub struct Chunk {
    pub tiles: [[[Tile; SIZE]; SIZE]; SIZE]
}

impl Chunk {
    pub fn generate(height_map: [[f32; SIZE]; SIZE]) -> Chunk {
        Chunk {
            tiles: array_16x16x16(|x, y, z| {
                let height = (height_map[x][z] * SIZE as f32) as usize;
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
