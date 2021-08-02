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
mod util;

pub use terrain::{generate_map, generate_precise_map, NoiseSample, Sample};
pub use util::array_int_to_float;
