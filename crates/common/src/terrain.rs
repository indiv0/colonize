use building_blocks::{
    mesh::{IsOpaque, MergeVoxel},
    storage::IsEmpty,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VoxelType {
    Air,
    Stone,
    Grass,
    Gold,
    Water,
}

impl VoxelType {
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