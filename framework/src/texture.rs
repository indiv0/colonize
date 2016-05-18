use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;

use image;
use image::{DynamicImage, GenericImage};
use glium;
use glium::backend::Facade;
use glium::texture::{RawImage2d, Texture2dDataSource};
use glium::uniforms::{AsUniformValue, UniformValue};

use manager::Resource;

pub type TextureData = glium::texture::CompressedSrgbTexture2d;

pub struct Texture {
    pub height: u32,
    pub width: u32,
    pub data: TextureData,
}

impl Texture {
    pub fn new<'a, F, T>(facade: &F, source: T) -> Self
        where F: Facade,
              T: Texture2dDataSource<'a>,
    {
        let texture = TextureData::new(facade, source).unwrap();
        Texture {
            width: texture.get_width(),
            height: texture.get_height().unwrap(),
            data: texture,
        }
    }

    pub fn from_image<F>(facade: &F, image: &DynamicImage) -> Texture
        where F: Facade,
    {
        Texture::new(facade, RawImage2d::from_raw_rgba_reversed(
                image.raw_pixels(),
                image.dimensions(),
            )
        )
    }
}

impl Resource for Texture {
    fn load<F>(facade: &F, path: &Path) -> Self
        where F: Facade,
    {
        let image = image::open(path).unwrap();
        Self::from_image(facade, &image)
    }
}

/// Texture reference.
#[derive(Clone)]
pub struct TextureRef(pub Rc<Texture>);

impl AsUniformValue for TextureRef {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::CompressedSrgbTexture2d(&self.0.data, None)
    }
}

impl Deref for TextureRef {
    type Target = Texture;

    fn deref(&self) -> &Texture {
        self.0.deref()
    }
}
