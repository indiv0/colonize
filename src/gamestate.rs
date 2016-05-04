use backend::TcodRenderer;

pub struct GameState {
    // TODO: replace this with RendererTrait`
    renderer: TcodRenderer,
}

impl GameState {
    pub fn new(renderer: TcodRenderer) -> GameState {
        GameState {
            renderer: renderer,
        }
    }

    pub fn get_renderer(&mut self) -> &mut TcodRenderer {
        &mut self.renderer
    }
}
