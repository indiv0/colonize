use array::Array;
use cgmath::Point3;
use noise::{ GenFn2, Seed };
use num;
use num::{ Float, NumCast };

use CHUNK_SIZE;
use chunk::Chunk;

pub fn generate_chunk<F>(pos: Point3<i32>, height_map: [[f64; CHUNK_SIZE]; CHUNK_SIZE], mut set_chunk: F)
    where F: FnMut(Point3<i32>, Chunk),
{
    set_chunk(pos, Chunk::generate(pos, height_map));
}

/// Generates a 2D height map at the specified location.
pub fn generate_height_map<T, F>(seed: &Seed, pos: &Point3<i32>, rng: F) -> [[T; CHUNK_SIZE]; CHUNK_SIZE]
    where T: Float + NumCast,
          F: GenFn2<T>,
{
    let get_random_height = |x, z| {
        let loc = [
            cast(x as i32 + pos.x * CHUNK_SIZE as i32),
            cast(z as i32 + pos.z * CHUNK_SIZE as i32),
        ];
        let value: f64 = cast(rng(seed, &loc));
        cast(clamp(value * 0.5 + 0.5, 0.0, 1.0))
    };
    array_16x16(get_random_height)
}

fn array_16x16<T, F>(mut f: F) -> [[T; CHUNK_SIZE]; CHUNK_SIZE]
    where F: FnMut(usize, usize) -> T
{
    Array::from_fn(|z| -> [T; CHUNK_SIZE] {
        Array::from_fn(|x| f(x, z))
    })
}

fn cast<T, R>(val: T) -> R
    where R: NumCast,
          T: NumCast,
{
    num::cast(val).unwrap()
}

fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}
