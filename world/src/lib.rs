#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate array;
extern crate cgmath;
extern crate noise;
extern crate num;
extern crate rand;
extern crate colonize_utility as utility;

// CHUNK_SIZE needs to be a power of two.
// TODO: possibly find a way to make `CHUNK_SIZE` configurable at runtime.
// TODO: possibly find a way to assert that `CHUNK_SIZE` is a power of two.
pub const CHUNK_SIZE: usize = 16;
// This should always be the log base 2 of `CHUNK_SIZE`.
// TODO: find a way to generate this at runtime.
pub const LOG2_OF_CHUNK_SIZE: u32 = 4;
// The multiplier by which the generated height maps are multiplied.
pub const HEIGHT_MAP_MULTIPLIER: f64 = 32.0;

pub use self::area::abs_pos_to_chunk_pos;
pub use self::chunk::Chunk;
pub use self::direction::Direction;
pub use self::terrain::TileType;
pub use self::world::World;

mod area;
mod chunk;
mod direction;
mod mapgen;
mod terrain;
mod world;
