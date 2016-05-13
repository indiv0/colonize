use std::collections::HashMap;
use std::rc::Rc;

use piston::input::{GenericEvent, PressEvent};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use rgframework::{BoxedScene, Scene, SceneCommand};
use rgframework::backend::{Backend, Graphics};
use rgframework::backend::graphics::Context;

use config::Config;
use localization::Localization;
use scene::GameScene;
use textures::TextureType;

pub struct MenuScene<B>
    where B:Backend,
{
    config: Rc<Config>,
    localization: Rc<Localization>,
    textures: Rc<HashMap<TextureType, B::Texture>>,
}

impl<B> MenuScene<B>
    where B: Backend,
{
    pub fn new(config: Rc<Config>, localization: Rc<Localization>, textures: Rc<HashMap<TextureType, B::Texture>>) -> Self {
        MenuScene {
            config: config,
            localization: localization,
            textures: textures,
        }
    }
}

impl<B, E, G> Scene<B, E, G> for MenuScene<B>
    where B: Backend + 'static,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
{
    fn to_box(self) -> BoxedScene<B, E, G> {
        Box::new(self)
    }

    fn render(&mut self, context: &Context, graphics: &mut G, glyph_cache: &mut B::CharacterCache) {
        use graphics::{clear, color, Transformed};
        use graphics::text::Text;

        clear(color::WHITE, graphics);

        Text::new(self.config.font_size).draw(
            &self.localization.menuscene_singleplayer,
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 100.0),
            graphics);

        Text::new(self.config.font_size).draw(
            &self.localization.menuscene_options,
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 150.0),
            graphics);

        Text::new(self.config.font_size).draw(
            &self.localization.menuscene_credits,
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 200.0),
            graphics);
    }

    fn handle_event(&mut self, e: &E) -> Option<SceneCommand<B, E, G>> {
        let mut maybe_scene = None;

        e.press(|button_type| {
            if let Keyboard(Key::S) = button_type {
                maybe_scene = Some(SceneCommand::SetScene(GameScene::new(self.config.clone(), self.localization.clone(), self.textures.clone()).to_box()));
            }
        });

        maybe_scene
    }
}
