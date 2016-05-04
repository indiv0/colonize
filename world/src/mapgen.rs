use array::Array;
use noise::{ GenFn2, Seed };
use num;
use num::{ Float, NumCast };

use CHUNK_SIZE;
use chunk::Chunk;

pub fn generate_chunk<T, F, G>(seed: &Seed, pos_x: i32, pos_z: i32, rng: F, mut set_chunk: G)
    where T: Float + NumCast,
          F: GenFn2<T>,
          G: FnMut((i32, i32), Chunk),
{
    let get_random_height = |x, z| {
        let loc = [
            cast(x as i32 + pos_x * CHUNK_SIZE as i32),
            cast(z as i32 + pos_z * CHUNK_SIZE as i32),
        ];
        let value: f64 = cast(rng(seed, &loc));
        cast(clamp(value * 0.5 + 0.5, 0.0, 1.0))
    };
    let height_map = array_16x16(get_random_height);

    let chunk = Chunk::generate(height_map);

    set_chunk((pos_x, pos_z), chunk);
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
