use std::collections::VecDeque;

use bevy::log::trace;
use building_blocks::{
    core::{Extent2i, Extent3i, Point3i, PointN},
    storage::{Array2, Array3, ForEachMut, Get, GetMut},
};
use colonize_common::{Voxel, VoxelType, EMPTY_VOXEL};

use crate::array_int_to_float;

pub fn generate_map<H, D>(
    elevation_noise: &H,
    dirt_thickness_noise: &D,
    sea_level: i32,
    minimum: Point3i,
    shape: Point3i,
) -> Array3<Voxel>
where
    D: Sample<[f64; 2], f64>,
    H: Sample<[f64; 2], f64>,
{
    // Generate the 2D height map.
    trace!("Generating 2D height map");
    let extent = Extent2i::from_min_and_shape(minimum.xz(), shape.xz());
    let filler = |point: &PointN<[i32; 2]>| {
        const MIN_WATER_LEVEL: f64 = -128.;
        const MAX_MOUNTAIN_HEIGHT: f64 = 128.;
        let sample = elevation_noise.get(array_int_to_float(point.0));
        scale(sample, -1., 1., MIN_WATER_LEVEL, MAX_MOUNTAIN_HEIGHT).round() as i32
    };
    let height_array = Array2::fill_with(extent, filler);

    // Generate the 3D strata map from the height map.
    let total_extent = Extent3i::from_min_and_shape(minimum, shape);
    trace!("Generating 3D strata map for extent {:?}", total_extent);
    let mut strata_array = Array3::fill(total_extent, VoxelType::Air);
    // Construct an iterator over 1x1-sized columns of the entire map.
    // We need to do this because we calculate once per columns: dirt thickness,
    // dirt transition, and stone transition.
    let column_extents = (total_extent.minimum.z()..=total_extent.max().z())
        .flat_map(move |z| (total_extent.minimum.x()..=total_extent.max().x()).map(move |x| (x, z)))
        .map(move |(x, z)| {
            let column_minimum = PointN([x, total_extent.minimum.y(), z]);
            let column_shape = PointN([1, shape.y(), 1]);
            Extent3i::from_min_and_shape(column_minimum, column_shape)
        });
    column_extents.for_each(|c| {
        let point = c.minimum.xz().0;
        let dirt_thickness = scale(
            dirt_thickness_noise.get(array_int_to_float(point)),
            -1.,
            1.,
            -2.0,
            5.,
        ) as i32;
        let dirt_transition = height_array.get(&PointN(point));
        let stone_transition = dirt_transition - dirt_thickness;
        strata_array.for_each_mut(&c, |point: Point3i, value| {
            if point.y() <= stone_transition {
                *value = VoxelType::Stone
            } else if point.y() <= dirt_transition {
                *value = VoxelType::Grass
            } else {
                *value = VoxelType::Air
            }
        })
    });

    // Flood-fill the water on the map.
    trace!("Flood-filling water on map");
    let maximum = total_extent.max();
    let water_generator = WaterGenerator::new(
        sea_level,
        minimum.x(),
        maximum.x() + 1,
        minimum.y(),
        minimum.z(),
        maximum.z() + 1,
    );
    water_generator.flood_fill(&mut strata_array);

    // Copy the 3D terrain map to a 3D density map. This is effectively an SDF map where the
    // signed distance is the distance of the voxel from the surface of the heightmap.
    let mut sdf_array = Array3::fill(total_extent, EMPTY_VOXEL);
    let column_extents = (total_extent.minimum.z()..=total_extent.max().z())
        .flat_map(move |z| (total_extent.minimum.x()..=total_extent.max().x()).map(move |x| (x, z)))
        .map(move |(x, z)| {
            let column_minimum = PointN([x, total_extent.minimum.y(), z]);
            let column_shape = PointN([1, shape.y(), 1]);
            Extent3i::from_min_and_shape(column_minimum, column_shape)
        });
    column_extents.for_each(|c| {
        let _height = (total_extent.minimum.y()..(total_extent.minimum.y() + shape.y()))
            .take_while(|y| {
                let p = PointN([c.minimum.x(), *y, c.minimum.z()]);
                let voxel_type = strata_array.get(&p);
                voxel_type != VoxelType::Air
            })
            .last()
            .unwrap();
        sdf_array.for_each_mut(&c, |point: Point3i, value| {
            *value = Voxel::new(strata_array.get(&point));
        })
    });

    sdf_array
}

struct WaterGenerator {
    sea_level: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    min_z: i32,
    max_z: i32,
    src: VoxelType,
    dst: VoxelType,
}

impl WaterGenerator {
    pub fn new(sea_level: i32, min_x: i32, max_x: i32, min_y: i32, min_z: i32, max_z: i32) -> Self {
        Self {
            sea_level,
            min_x,
            max_x,
            min_y,
            min_z,
            max_z,
            src: VoxelType::Air,
            dst: VoxelType::Water,
        }
    }

    pub fn flood_fill(&self, array: &mut Array3<VoxelType>) {
        self.flood_fill_horizontal(array, self.min_x, self.min_z);
        self.flood_fill_horizontal(array, self.min_x, self.max_z - 1);
        self.flood_fill_horizontal(array, self.max_x - 1, self.min_z);
        self.flood_fill_horizontal(array, self.max_x - 1, self.max_z - 1);
    }

    fn flood_fill_horizontal(&self, array: &mut Array3<VoxelType>, x: i32, z: i32) {
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
                let voxel = array.get_mut(&west);
                if *voxel == self.src {
                    *voxel = self.dst;
                    queue.push_back(west);
                }
            }
            if n.x() > self.min_x {
                let east = PointN([n.x() - 1, n.y(), n.z()]);
                let voxel = array.get_mut(&east);
                if *voxel == self.src {
                    *voxel = self.dst;
                    queue.push_back(east);
                }
            }
            if n.z() < self.max_z - 1 {
                let north = PointN([n.x(), n.y(), n.z() + 1]);
                let voxel = array.get_mut(&north);
                if *voxel == self.src {
                    *voxel = self.dst;
                    queue.push_back(north);
                }
            }
            if n.z() > self.min_x {
                let south = PointN([n.x(), n.y(), n.z() - 1]);
                let voxel = array.get_mut(&south);
                if *voxel == self.src {
                    *voxel = self.dst;
                    queue.push_back(south);
                }
            }
            if n.y() > self.min_y {
                let down = PointN([n.x(), n.y() - 1, n.z()]);
                let voxel = array.get_mut(&down);
                if *voxel == self.src {
                    *voxel = self.dst;
                    queue.push_back(down);
                }
            }
        }
    }
}

pub trait Sample<T, U> {
    fn get(&self, point: T) -> U;
}

pub trait NoiseSample<T, U>: Sample<T, U> {}

/// Scales a value in the range [a_min, a_max] to the range [b_min, b_max]
fn scale(number: f64, a_min: f64, a_max: f64, b_min: f64, b_max: f64) -> f64 {
    let scaled = (((number - a_min) * (b_max - b_min)) / (a_max - a_min)) + b_min;
    assert!(
        number >= a_min,
        "{} must be greater than or equal to {}",
        number,
        a_min
    );
    assert!(
        number <= a_max,
        "{} must be less than or equal to {}",
        number,
        a_max
    );
    assert!(
        scaled >= b_min,
        "{} must be greater than or equal to {}",
        scaled,
        b_min
    );
    assert!(
        scaled <= b_max,
        "{} must be less than or equal to {}",
        scaled,
        b_max
    );
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
}
