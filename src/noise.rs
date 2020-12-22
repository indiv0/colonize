use std::{marker::PhantomData, ops::Range};

use noise::NoiseFn;

pub(crate) struct Noise2d<N, T>
    where N: NoiseFn<T>,
{
    noise: N,
    marker: PhantomData<T>,
}

impl<N, T> Noise2d<N, T>
    where N: NoiseFn<T>,
{
    pub(crate) fn new(noise: N) -> Self {
        Self {
            noise,
            marker: PhantomData,
        }
    }
}

pub(crate) trait HeightMapSampler<T> {
    fn sample_elevation(&self, point: T) -> i32;
}

impl<N, T> HeightMapSampler<T> for Noise2d<N, T>
    where N: NoiseFn<T>,
{
    fn sample_elevation(&self, point: T) -> i32 {
        const MIN_WATER_LEVEL: f64 = -128.;
        const MAX_MOUNTAIN_HEIGHT: f64 = 128.;
        let sample = self.noise.get(point);
        scale(sample, -1., 1., MIN_WATER_LEVEL, MAX_MOUNTAIN_HEIGHT).round() as i32
    }
}

pub(crate) trait DirtThicknessSampler<T> {
    fn sample_dirt_thickness(&self, point: T) -> f64;
}

impl<N, T> DirtThicknessSampler<T> for Noise2d<N, T>
    where N: NoiseFn<T>,
{
    fn sample_dirt_thickness(&self, point: T) -> f64 {
        let sample = self.noise.get(point);
        scale(sample, -1., 1., -2.0, 5.)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CubeVoxel {
    Air,
    Stone,
    Grass,
    Gold,
    Water,
}

pub(crate) struct VoxelColumn {
    dirt_transition: i32,
    stone_transition: i32,
    range: Range<i32>,
}

impl VoxelColumn {
    pub(crate) fn new<D, H, T, DT>(dirt_thickness_sampler: &D, height_map_sampler: &H, point: [T; 2], min_y: i32, max_y: i32) -> Self
        where D: DirtThicknessSampler<[DT; 2]>,
              H: HeightMapSampler<[T; 2]>,
              T: Copy + Into<DT>,
    {
        let dirt_thickness = dirt_thickness_sampler.sample_dirt_thickness([point[0].into(), point[1].into()]) as i32;
        let dirt_transition = height_map_sampler.sample_elevation(point) as i32;
        let stone_transition = dirt_transition - dirt_thickness;
        Self {
            dirt_transition,
            stone_transition,
            range: (min_y..max_y)
        }
    }
}

impl Iterator for VoxelColumn {
    type Item = CubeVoxel;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|y| self.sample_voxel(self.stone_transition, self.dirt_transition, y))
    }
}


trait StrataSampler {
    fn sample_voxel(&self, stone_transition: i32, dirt_transition: i32, y: i32) -> CubeVoxel {
        if y <= stone_transition {
            CubeVoxel::Stone
        } else if y <= dirt_transition {
            CubeVoxel::Grass
        } else {
            CubeVoxel::Air
        }
    }
}
impl StrataSampler for VoxelColumn {}

/// Scales a value in the range [a_min, a_max] to the range [b_min, b_max]
fn scale(number: f64, a_min: f64, a_max: f64, b_min: f64, b_max: f64) -> f64 {
    let scaled = (((number - a_min) * (b_max - b_min)) / (a_max - a_min)) + b_min;
    assert!(number >= a_min, "{} must be greater than or equal to {}", number, a_min);
    assert!(number <= a_max, "{} must be less than or equal to {}", number, a_max);
    assert!(scaled >= b_min, "{} must be greater than or equal to {}", scaled, b_min);
    assert!(scaled <= b_max, "{} must be less than or equal to {}", scaled, b_max);
    scaled
}

// Prevent users from implementing the HeightMap trait.
mod private {
    pub trait Sealed {}
}