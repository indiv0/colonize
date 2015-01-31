use utility::Point;
use windowing::WindowTrait;

#[derive(Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub trait RendererTrait<W: WindowTrait> {
    fn before_render(&mut self);
    fn render_obj(&mut self, position: Point, symbol: char);
    fn render_frame(&mut self);
    fn attach_window(&mut self, window: &mut W);
}
