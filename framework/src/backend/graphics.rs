use std::rc::Rc;

use glium::backend::Context;

pub struct RenderContext {
    pub context: Rc<Context>,
}
