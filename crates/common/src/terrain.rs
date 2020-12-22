use building_blocks::{mesh::MaterialVoxel, storage::IsEmpty};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CubeVoxel {
    Air,
    Stone,
    Grass,
    Gold,
    Water,
}

impl MaterialVoxel for CubeVoxel {
    type Material = u8;

    fn material(&self) -> Self::Material {
        match self {
            // Technically air doesn't have a material since it doesn't get rendered, but we need to
            // provide _something_ here.
            CubeVoxel::Air => 0,
            CubeVoxel::Stone => 0,
            CubeVoxel::Grass => 1,
            CubeVoxel::Gold => 2,
            CubeVoxel::Water => 3,
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