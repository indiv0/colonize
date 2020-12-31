use building_blocks::{mesh::{IsOpaque, MergeVoxel}, storage::IsEmpty};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CubeVoxel {
    Air,
    Stone,
    Grass,
    Gold,
    Water,
}

impl CubeVoxel {
    pub fn collidable(&self) -> bool {
        match self {
            CubeVoxel::Air | CubeVoxel::Water => false,
            CubeVoxel::Stone | CubeVoxel::Grass | CubeVoxel::Gold => true,
        }
    }
}

impl MergeVoxel for CubeVoxel {
    type VoxelValue = Self;

    fn voxel_merge_value(&self) -> Self::VoxelValue {
        *self
    }
}

impl IsOpaque for CubeVoxel {
    fn is_opaque(&self) -> bool {
        match self {
            CubeVoxel::Air | CubeVoxel::Water => true,
            CubeVoxel::Stone | CubeVoxel::Grass | CubeVoxel::Gold => false,
        }
    }
}

impl IsEmpty for CubeVoxel {
    fn is_empty(&self) -> bool {
        match self {
            CubeVoxel::Air => true,
            CubeVoxel::Stone | CubeVoxel::Grass | CubeVoxel::Gold | CubeVoxel::Water => false,
        }
    }
}