mod terrain;
mod util;

pub use terrain::{generate_height_map, generate_strata_map, NoiseSample, Sample, WaterGenerator};
pub use util::array_int_to_float;