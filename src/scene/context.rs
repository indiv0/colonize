use std::cell::RefCell;
use std::rc::Rc;

use glium::backend::Facade;
use rgframework::manager::Manager;
use rgframework::rendering::{sprite, Renderer, TextRenderer};
use rgframework::texture::Texture;

use config::Config;
use localization::Localization;

pub struct SceneContext<F>
    where F: Facade,
{
    pub config: Config,
    pub facade: Rc<F>,
    pub localization: Rc<Localization>,
    pub sprite_renderer: Renderer<F, sprite::Shader>,
    pub textures: Manager<F, Texture>,
    pub text_renderer: RefCell<TextRenderer>,
}

impl<F> SceneContext<F>
    where F: Facade,
{
    pub fn new(
        config: Config,
        localization: Rc<Localization>,
        facade: F,
        sprite_renderer: Renderer<F, sprite::Shader>,
        textures: Manager<F, Texture>,
        text_renderer: TextRenderer,
    ) -> Self {
        SceneContext {
            config: config,
            localization: localization,
            facade: Rc::new(facade),
            sprite_renderer: sprite_renderer,
            textures: textures,
            text_renderer: RefCell::new(text_renderer),
        }
    }
}
