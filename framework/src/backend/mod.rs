pub use self::graphics::{CharacterCache, Graphics};
use std;

pub mod graphics;

pub trait Backend {
    type Texture: self::graphics::ImageSize + std::any::Any;
    type CharacterCache: CharacterCache<Texture=Self::Texture>;
}

impl<T, C> Backend for (T, C)
    where T: self::graphics::ImageSize + std::any::Any,
          C: CharacterCache<Texture=T>,
{
    type Texture = T;
    type CharacterCache = C;
}
