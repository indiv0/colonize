pub const CHUNK_SIZE: usize = 16;

pub use self::world::World;
pub use self::chunk::Chunk;

mod chunk;
mod world;
