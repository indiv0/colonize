#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    unused,
    unused_extern_crates,
    clippy::all
)]
mod terrain;

pub use terrain::{Voxel, VoxelDistance, VoxelType, EMPTY_VOXEL, NUM_VOXEL_TYPES};
