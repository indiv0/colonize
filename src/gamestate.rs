use backend::Renderer;

pub struct GameState {
    renderer: Renderer,
}

impl GameState {
    pub fn new(renderer: Renderer) -> GameState {
        GameState {
            renderer: renderer,
        }
    }

    pub fn get_renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
