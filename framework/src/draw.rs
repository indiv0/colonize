use backend::{Backend, Graphics};
use backend::graphics::Context;

pub trait Draw<B, G>
    where B: Backend,
          G: Graphics<Texture=B::Texture>,
{
    fn draw(&self, context: &Context, graphics: &mut G, glyph_cache: &mut B::CharacterCache);
}
