use std::rc::Rc;

use cgmath::Vector2;
use glium::Surface;
use glium::backend::Facade;
use piston::input::{GenericEvent, PressEvent};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use rgframework::{BoxedScene, Scene, SceneCommand};
use rgframework::backend::graphics::RenderContext;
use rgframework::color;
use rgframework::color::Color;

use scene::{GameScene, SceneContext};

const BACKGROUND_COLOR: Color = color::WHITE;
const TEXT_COLOR: Color = color::BLACK;

pub struct MenuScene<F>
    where F: Facade
{
    context: Rc<SceneContext<F>>,
}

impl<F> MenuScene<F>
    where F: Facade,
{
    pub fn new(context: Rc<SceneContext<F>>) -> Self {
        MenuScene {
            context: context,
        }
    }
}

impl<E, F, S> Scene<E, S> for MenuScene<F>
    where E: GenericEvent,
          F: Facade + 'static,
          S: Surface,
{
    fn to_box(self) -> BoxedScene<E, S> {
        Box::new(self)
    }

    fn render(&mut self, surface: &mut S, context: &RenderContext) {
        surface.clear_color(BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b, BACKGROUND_COLOR.a);

        for &(ref text_str, pos) in &[
            (&self.context.localization.menuscene_singleplayer, Vector2::new(10.0, 100.0)),
            (&self.context.localization.menuscene_options, Vector2::new(10.0, 150.0)),
            (&self.context.localization.menuscene_credits, Vector2::new(10.0, 200.0)),
        ] {
            self.context.text_renderer.borrow_mut().draw(&context.context, surface, text_str, pos, TEXT_COLOR);
        }
    }

    fn handle_event(&mut self, e: &E) -> Option<SceneCommand<E, S>> {
        let mut maybe_scene = None;

        e.press(|button_type| {
            if let Keyboard(Key::S) = button_type {
                maybe_scene = Some(SceneCommand::SetScene(GameScene::new(self.context.clone()).to_box()));
            }
        });

        maybe_scene
    }
}
