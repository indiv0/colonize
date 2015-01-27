use conrod::UiContext;
use opengl_graphics::Gl;

pub struct GameState {
    pub uic: UiContext,
    pub gl: Gl,
}

impl GameState {
    pub fn new(uic: UiContext, gl: Gl, asset_dir: &Path) -> GameState {
        GameState {
            uic: uic,
            gl: gl,
        }
    }

    pub fn get_drawables(&mut self) -> (&mut UiContext, &mut Gl) {
        (&mut self.uic, &mut self.gl)
    }

    pub fn get_uic(&mut self) -> &mut UiContext {
        &mut self.uic
    }
}
