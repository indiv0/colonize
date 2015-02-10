use std::collections::HashMap;
use std::num::{ self, NumCast };

use backend::Renderer;
use gfx_voxel::array::Array;
use noise::{ open_simplex2, Seed };
use utility::{ Bounds, Point };

use chunk;
use chunk::Chunk;

static SEED: u32 = 0;

pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    seed: Seed,
}

fn array_16x16<T, F>(mut f: F) -> [[T; 16]; 16]
    where F: FnMut(usize, usize) -> T
{
    Array::from_fn(|z| -> [T; 16]
        Array::from_fn(|x| f(x, z))
    )
}

fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num::cast(val).unwrap()
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::new(),
            seed: Seed::new(SEED),
        }
    }

    pub fn add_chunk(&mut self, x: i32, z: i32, c: Chunk) {
        self.chunks.insert((x, z), c);
    }

    pub fn generate_chunk(&self, pos_x: i32, pos_z: i32) -> Chunk {
        let height_map = array_16x16(|x, z| {
            open_simplex2(&self.seed, &[cast::<_, f32>(x as i32 + pos_x * chunk::SIZE as i32), cast::<_, f32>(z as i32 + pos_z * chunk::SIZE as i32)])
        });

        Chunk::generate(height_map)
    }

    pub fn render(&self, renderer: &mut Renderer, bounds: Bounds, camera_pos: Point, height: usize) {
        for (&(x, y), chunk) in self.chunks.iter() {
            let x_offset = x * chunk::SIZE as i32 + camera_pos.x;
            let z_offset = y * chunk::SIZE as i32 + camera_pos.y;

            if bounds.contains(Point { x: x_offset, y: z_offset }) {
                chunk.render(renderer, x_offset, z_offset, height);
            }
        }
    }
}
