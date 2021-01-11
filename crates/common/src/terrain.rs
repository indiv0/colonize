use building_blocks::{mesh::{IsOpaque, MergeVoxel, SignedDistance}, storage::IsEmpty};

pub const EMPTY_VOXEL: Voxel = Voxel {
    voxel_type: VoxelType::Air,
    distance: VoxelDistance(1),
};

pub const NUM_VOXEL_TYPES: usize = 5;

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    voxel_type: VoxelType,
    distance: VoxelDistance,
}

impl Voxel {
    pub fn new(voxel_type: VoxelType, distance: VoxelDistance) -> Self {
        Self { voxel_type, distance }
    }

    pub fn voxel_type(&self) -> &VoxelType {
        &self.voxel_type
    }

    pub fn distance(&self) -> &VoxelDistance {
        &self.distance
    }
}

impl SignedDistance for Voxel {
    fn distance(&self) -> f32 {
        self.distance.0 as f32
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VoxelType {
    Air,
    Stone,
    Grass,
    Gold,
    Water,
}

impl VoxelType {
    pub fn index(&self) -> usize {
        match self {
            VoxelType::Air => 0,
            VoxelType::Stone => 1,
            VoxelType::Grass => 2,
            VoxelType::Gold => 3,
            VoxelType::Water => 4,
        }
    }

    pub fn collidable(&self) -> bool {
        match self {
            VoxelType::Air | VoxelType::Water => false,
            VoxelType::Stone | VoxelType::Grass | VoxelType::Gold => true,
        }
    }
}

impl MergeVoxel for VoxelType {
    type VoxelValue = Self;

    fn voxel_merge_value(&self) -> Self::VoxelValue {
        *self
    }
}

impl IsOpaque for VoxelType {
    fn is_opaque(&self) -> bool {
        match self {
            VoxelType::Air | VoxelType::Water => false,
            VoxelType::Stone | VoxelType::Grass | VoxelType::Gold => true,
        }
    }
}

impl IsEmpty for VoxelType {
    fn is_empty(&self) -> bool {
        match self {
            VoxelType::Air => true,
            VoxelType::Stone | VoxelType::Grass | VoxelType::Gold | VoxelType::Water => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VoxelDistance(pub i8);