//! Unique identifier.

use rand;
use time;

/// Unique identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Id {
    random: u32,
    time: time::Timespec,
}

impl Id {
    pub fn new() -> Id {
        Id {
            random: rand::random(),
            time: time::now().to_timespec(),
        }
    }
}
