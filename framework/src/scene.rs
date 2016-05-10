use piston::input::GenericEvent;

use backend::{Backend, Graphics};
use backend::graphics::Context;

pub type BoxedScene<B, E, G>
        where B: Backend,
              E: GenericEvent,
              G: Graphics<Texture=B::Texture>
    = Box<Scene<B, E, G> + 'static>;

pub enum SceneCommand<B, E, G> {
    SetScene(BoxedScene<B, E, G>),
    PushScene(BoxedScene<B, E, G>),
    PopScene,
    Clear,
}

pub trait Scene<B, E, G>
    where B: Backend,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
{
    fn to_box(self) -> BoxedScene<B, E, G>;
    fn render(&mut self, context: &Context, graphics: &mut G, glyph_cache: &mut B::CharacterCache);
    fn handle_event(&mut self, e: &E) -> Option<SceneCommand<B, E, G>>;
}
