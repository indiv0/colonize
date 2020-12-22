use std::{collections::VecDeque, iter, ops::RangeInclusive};

use building_blocks::{core::{Extent2i, Extent3i, Point2i, Point3i, PointN}, storage::{Array2, Array3, ForEachMut, Get, GetMut}};
use colonize_common::CubeVoxel;

use crate::{array_int_to_float, util::array_float_to_int};

struct Extent3iColumns {
    iter: Box<dyn Iterator<Item=Extent3i>>,
}

impl Extent3iColumns {
    fn new(extent: &Extent3i) -> Self {
        let min_x = extent.minimum.x();
        let min_y = extent.minimum.y();
        let min_z = extent.minimum.z();
        let shape_y = extent.shape.y();
        let max_x = extent.max().x();
        let max_z = extent.max().z();
        let iter = (min_z..=max_z)
            .flat_map(move |z| (min_x..=max_x).map(move |x| (x, z)))
            .map(move |(x, z)| {
                let column_minimum = PointN([x, min_y, z]);
                let column_shape = PointN([1, shape_y, 1]);
                Extent3i::from_min_and_shape(column_minimum, column_shape)
            });
        Self {
            iter: Box::new(iter),
        }
    }
}

impl Iterator for Extent3iColumns {
    type Item = Extent3i;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub fn generate_height_map<H>(elevation_noise: &H, minimum: [i32; 2], shape: [i32; 2]) -> Array2<i32>
    where H: Sample<[f64; 2], f64>,
{
    let extent = Extent2i::from_min_and_shape(PointN(minimum), PointN(shape));
    let filler = |point: &PointN<[i32; 2]>| sample_elevation(elevation_noise, array_int_to_float(point.0));
    Array2::fill_with(extent, filler)
}

pub fn generate_strata_map<D, E>(dirt_thickness_noise: &D, elevation_sampler: &E, minimum: Point3i, shape: Point3i) -> Array3<CubeVoxel>
    where D: Sample<[f64; 2], f64>,
          E: Sample<[i32; 2], i32>,
{
    let total_extent = Extent3i::from_min_and_shape(minimum, shape);
    let maximum = total_extent.max();
    let mut array = Array3::fill(total_extent, CubeVoxel::Air);
    let x_range = minimum.x()..=maximum.x();
    let y_range = minimum.y()..=maximum.y();
    let z_range = minimum.z()..=maximum.z();
    let mut voxels = sample_space(dirt_thickness_noise, elevation_sampler, x_range, y_range, z_range);
    let column_extents = Extent3iColumns::new(&array.extent());
    column_extents.for_each(|c| {
        array.for_each_mut(&c, |_point: Point3i, value| {
            *value = voxels.next().unwrap()
        })
    });
    // The range covered by `sample_space` and the for-loop should be identical, so there
    // should be no remaining voxels.
    assert!(voxels.next().is_none());
    array
}

pub struct WaterGenerator {
    sea_level: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    min_z: i32,
    max_z: i32,
    src: CubeVoxel,
    dst: CubeVoxel,
}

impl WaterGenerator {
    pub fn new(sea_level: i32, min_x: i32, max_x: i32, min_y: i32, min_z: i32, max_z: i32) -> Self {
        Self { sea_level, min_x, max_x, min_y, min_z, max_z, src: CubeVoxel::Air, dst: CubeVoxel::Water }
    }

    pub fn flood_fill(&self, array: &mut Array3<CubeVoxel>) {
        self.flood_fill_horizontal(array, self.min_x, self.min_z);
        self.flood_fill_horizontal(array, self.min_x, self.max_z - 1);
        self.flood_fill_horizontal(array, self.max_x - 1, self.min_z);
        self.flood_fill_horizontal(array, self.max_x - 1, self.max_z - 1);
        //self.flood_fill_vertical(array);
    }

    fn flood_fill_horizontal(&self, array: &mut Array3<CubeVoxel>, x: i32, z: i32) {
        let location = PointN([x, self.sea_level - 1, z]);
        let voxel = array.get_mut(&location);
        if *voxel != self.src {
            return;
        }

        *voxel = self.dst;

        let mut queue = VecDeque::new();
        queue.push_back(location);

        while let Some(n) = queue.pop_front() {
            assert!(n.x() >= self.min_x);
            assert!(n.x() <= self.max_x);
            assert!(n.z() >= self.min_z);
            assert!(n.z() <= self.max_z);
            if n.x() < self.max_x - 1 {
                let west = PointN([n.x() + 1, n.y(), n.z()]);
                self.try_replace_single_voxel(array, west, &mut queue);
            }
            if n.x() > self.min_x {
                let east = PointN([n.x() - 1, n.y(), n.z()]);
                self.try_replace_single_voxel(array, east, &mut queue);
            }
            if n.z() < self.max_z - 1 {
                let north = PointN([n.x(), n.y(), n.z() + 1]);
                self.try_replace_single_voxel(array, north, &mut queue);
            }
            if n.z() > self.min_x {
                let south = PointN([n.x(), n.y(), n.z() - 1]);
                self.try_replace_single_voxel(array, south, &mut queue);
            }
            if n.y() > self.min_y {
                let down = PointN([n.x(), n.y() - 1, n.z()]);
                self.try_replace_single_voxel(array, down, &mut queue);
            }
        }
    }

    fn flood_fill_vertical(&self, array: &mut Array3<CubeVoxel>) {
        for z in self.min_z..self.max_z {
            for x in self.min_x..self.max_x {
                for y in (self.min_y..(self.sea_level - 1)).rev() {
                    let voxel = array.get_mut(&PointN([x, y, z]));
                    if *voxel == self.dst {
                        continue;
                    } else if *voxel == self.src {
                        *voxel = self.dst;
                        continue;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn try_replace_single_voxel(&self, array: &mut Array3<CubeVoxel>, location: Point3i, queue: &mut VecDeque<Point3i>) {
        let voxel = array.get_mut(&location);
        if *voxel == self.src {
            *voxel = self.dst;
            queue.push_back(location);
        }
    }
}

fn sample_space<'a, D, E>(dirt_thickness_noise: &'a D, elevation_sampler: &'a E, x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>, z_range: RangeInclusive<i32>) -> impl Iterator<Item=CubeVoxel> + 'a
    where D: Sample<[f64; 2], f64>,
          E: Sample<[i32; 2], i32>,
{
    z_range
        .flat_map(move |z| x_range.clone().zip(iter::repeat(z)))
        .map(|(x, z)| [x as f64, z as f64])
        .flat_map(move |point| {
            sample_column(dirt_thickness_noise, elevation_sampler, point, y_range.clone())
        })
}

fn sample_column<'a, D, E>(dirt_thickness_noise: &'a D, elevation_sampler: &'a E, point: [f64; 2], y_range: RangeInclusive<i32>) -> impl Iterator<Item=CubeVoxel> + 'a
    where D: Sample<[f64; 2], f64>,
          E: Sample<[i32; 2], i32>,
{
    let dirt_thickness = sample_dirt_thickness(dirt_thickness_noise, point) as i32;
    let dirt_transition = elevation_sampler.get(array_float_to_int(point));
    let stone_transition = dirt_transition - dirt_thickness;
    y_range.map(move |y| sample_voxel(stone_transition, dirt_transition, y))
}

fn sample_dirt_thickness<S, T>(noise: &S, point: T) -> f64
    where S: Sample<T, f64>,
{
    let sample = noise.get(point);
    scale(sample, -1., 1., -2.0, 5.)
}

fn sample_elevation<S, T>(noise: &S, point: T) -> i32
    where S: Sample<T, f64>,
{
    const MIN_WATER_LEVEL: f64 = -128.;
    const MAX_MOUNTAIN_HEIGHT: f64 = 128.;
    let sample = noise.get(point);
    scale(sample, -1., 1., MIN_WATER_LEVEL, MAX_MOUNTAIN_HEIGHT).round() as i32
}

fn sample_voxel(stone_transition: i32, dirt_transition: i32, y: i32) -> CubeVoxel {
    if y <= stone_transition {
        CubeVoxel::Stone
    } else if y <= dirt_transition {
        CubeVoxel::Grass
    } else {
        CubeVoxel::Air
    }
}

pub trait Sample<T, U> {
    fn get(&self, point: T) -> U;
}

pub trait NoiseSample<T, U>: Sample<T, U> {}

impl Sample<[i32; 2], i32> for Array2<i32>
{
    fn get(&self, point: [i32; 2]) -> i32 {
        // FIXME: it's not safe to assume that [i32; 2] corresponds to a Point2i.
        //   It could also be a Local or a Stride.
        Get::get(self, &PointN(point))
    }
}

/// Scales a value in the range [a_min, a_max] to the range [b_min, b_max]
fn scale(number: f64, a_min: f64, a_max: f64, b_min: f64, b_max: f64) -> f64 {
    let scaled = (((number - a_min) * (b_max - b_min)) / (a_max - a_min)) + b_min;
    assert!(number >= a_min, "{} must be greater than or equal to {}", number, a_min);
    assert!(number <= a_max, "{} must be less than or equal to {}", number, a_max);
    assert!(scaled >= b_min, "{} must be greater than or equal to {}", scaled, b_min);
    assert!(scaled <= b_max, "{} must be less than or equal to {}", scaled, b_max);
    scaled
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockNoise;

    impl Sample<[f64; 2], f64> for MockNoise {
        fn get(&self, _point: [f64; 2]) -> f64 {
            1.0
        }
    }

    impl Sample<[i32; 2], i32> for MockNoise {
        fn get(&self, _point: [i32; 2]) -> i32 {
            1
        }
    }

    #[test]
    fn test_extent3i_columns() {
        let minimum = PointN([-4, -4, -4]);
        let shape = PointN([8, 8, 8]);
        let extent = Extent3i::from_min_and_shape(minimum, shape);
        let column_extents = Extent3iColumns::new(&extent);
        assert_eq!(column_extents.count(), 8_usize.pow(2));
    }

    #[test]
    fn test_sample_space() {
        let dirt_thickness_noise = MockNoise;
        let elevation_sampler = MockNoise;
        let x_range = -4..=3;
        let y_range = -4..=3;
        let z_range = -4..=3;
        let count = sample_space(&dirt_thickness_noise, &elevation_sampler, x_range, y_range, z_range).count();
        assert_eq!(count, usize::pow(8, 3));
    }

    #[test]
    fn test_sample_column() {
        let dirt_thickness_noise = MockNoise;
        let elevation_sampler = MockNoise;
        let point = [0.; 2];
        let y_range = -4..=3;
        let count = sample_column(&dirt_thickness_noise, &elevation_sampler, point, y_range).count();
        assert_eq!(count, 8);
    }
}