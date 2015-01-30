use std::collections::HashMap;
use std::num::{self, Float, NumCast};

use gfx_voxel::array::Array;
use graphics::Context;
use noise::{open_simplex2, Seed};
use opengl_graphics::Gl;

use chunk::Chunk;

static SEED: u32 = 0;
static SIZE: usize = 4;

pub struct Map {
    chunks: HashMap<(i32, i32), Chunk>,
    seed: Seed,
}

fn array_16x16(mut f: F) -> [[f32; 16]; 16]
    where F: FnMut(usize, usize) -> T
{
    Array::from_fn(|z| -> [f32; 16]
        Array::from_fn(|x| -> f(x, z))
    )
}

fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num::cast(val).unwrap()
}

impl Map {
    pub fn new() -> Map {
        Map {
            chunks: HashMap::new(),
            seed: Seed::new(SEED),
        }
    }

    pub fn add_chunk(&mut self, x: i32, z: i32, c: Chunk) {
        self.chunks.insert((x, z), c);
    }

    pub fn generate_chunk(&self) -> Chunk {
        let height_map = array_16x16(|x, z| {
            open_simplex2(&self.seed, &[cast::<_, f32>(x), cast::<_, f32>(z)]);
        });
    }
}
