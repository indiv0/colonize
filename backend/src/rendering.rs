extern crate tcod;

use cgmath::Point2;
use self::tcod::Color;

use windowing::Window;

#[derive(Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub trait Renderer<W: Window> {
    fn before_render(&mut self);
    fn render_obj(&mut self, position: Point2<i32>, symbol: char, color: Option<Color>);
    fn render_frame(&mut self);
    fn attach_window(&mut self, window: &mut W);
}
