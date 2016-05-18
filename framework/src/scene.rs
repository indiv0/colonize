use glium::Surface;
use piston::input::GenericEvent;

use backend::graphics::RenderContext;

pub type BoxedScene<E, S>
        where E: GenericEvent,
              S: Surface
    = Box<Scene<E, S> + 'static>;

pub enum SceneCommand<E, S> {
    SetScene(BoxedScene<E, S>),
    PushScene(BoxedScene<E, S>),
    PopScene,
    Clear,
}

pub trait Scene<E, S>
    where E: GenericEvent,
          S: Surface,
{
    fn to_box(self) -> BoxedScene<E, S>;
    fn render(&mut self, surface: &mut S, context: &RenderContext);
    fn handle_event(&mut self, e: &E) -> Option<SceneCommand<E, S>>;
}
