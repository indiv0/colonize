use std::collections::HashMap;
use std::path::PathBuf;

use rgframework::backend::Backend;
use opengl_graphics::Texture;
use world::TileType;

use backend::GlBackend;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum TextureType {
    TileTexture(TileType),
}

pub fn load_textures_opengl(textures_path: &PathBuf) -> HashMap<TextureType, <GlBackend as Backend>::Texture> {
    use self::TextureType::TileTexture;

    let mut textures = HashMap::new();
    for &(ref texture_type, file_name) in &[
        (TileTexture(TileType::Grass), "game_scene/grass.png"),
        (TileTexture(TileType::Sand), "game_scene/sand.png"),
        (TileTexture(TileType::Soil), "game_scene/soil.png"),
        (TileTexture(TileType::Wall), "game_scene/wall.png"),
        (TileTexture(TileType::Water), "game_scene/water.png"),
    ] {
        textures.insert((*texture_type).clone(), Texture::from_path(textures_path.join(file_name)).unwrap());
    }

    textures
}
