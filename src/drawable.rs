use backend::TcodRenderer;
use cgmath::Point2;

pub trait Drawable {
    fn draw(&self, renderer: &mut TcodRenderer, offset: Point2<i32>);
}
