use rand;
use rand::Rng;

use area::Area;

pub struct World {
    pub area: Area,
}

impl World {
    pub fn new(seed: Option<u32>, initial_size: u32) -> Self {
        // Use system RNG for seed if the user didn't provide one.
        let seed = seed.unwrap_or(rand::thread_rng().gen());

        World {
            area: Area::new(seed, initial_size),
        }
    }
}
