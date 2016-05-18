use cgmath::Vector2;
use world::TileType;

pub type Offset = Vector2<u32>;

pub trait GetOffset {
    fn offset(&self) -> Offset;
}

pub enum TextureType {
    Tile(TileType),
    Cursor,
}

impl GetOffset for TileType {
    #[inline]
    fn offset(&self) -> Offset {
        use world::TileType::*;

        match *self {
            Grass => Vector2::new(0, 0),
            Sand => Vector2::new(1, 0),
            Soil => Vector2::new(2, 0),
            Wall => Vector2::new(3, 0),
            Water => Vector2::new(0, 1),
            Air | OutOfBounds => Vector2::new(0, 0),
        }
    }
}

impl GetOffset for TextureType {
    #[inline]
    fn offset(&self) -> Offset {
        use self::TextureType::*;

        match *self {
            Tile(tile_type) => tile_type.offset(),
            Cursor => Vector2::new(1, 1),
        }
    }
}
