#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    future_incompatible,
    unused,
    unused_extern_crates,
    clippy::all
)]
use std::marker::PhantomData;

use colonize_core::{NoiseSample, Sample};
use noise::NoiseFn;

pub struct Noise2d<N, T>
where
    N: NoiseFn<T>,
{
    noise: N,
    marker: PhantomData<T>,
}

impl<N, T> Noise2d<N, T>
where
    N: NoiseFn<T>,
{
    pub fn new(noise: N) -> Self {
        Self {
            noise,
            marker: PhantomData,
        }
    }
}

impl<N, T> NoiseSample<T, f64> for Noise2d<N, T> where N: NoiseFn<T> {}

impl<N, T> Sample<T, f64> for Noise2d<N, T>
where
    N: NoiseFn<T>,
{
    fn get(&self, point: T) -> f64 {
        self.noise.get(point)
    }
}
