use conrod::{
    Background,
    Color,
    Colorable,
    Drawable,
    UiContext,
};
use event::{RenderArgs, UpdateArgs};
use input::Button;
use opengl_graphics::Gl;

/// The current state the game is in.
enum PlayState {
    MainMenu
}

pub struct App {
    state: PlayState,
    uic: UiContext,
    bg_color: Color,
}

impl App {
    pub fn new(uic: UiContext) -> App {
        App {
            state: PlayState::MainMenu,
            uic: uic,
            bg_color: Color::new(0.2, 0.35, 0.45, 1.0),
        }
    }

    pub fn load(&self) {
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
    }

    pub fn key_press(&mut self, _button: Button) {
    }

    pub fn key_release(&mut self, _button: Button) {
    }

    pub fn render(&mut self, _args: &RenderArgs, gl: &mut Gl) {
        match self.state {
            PlayState::MainMenu => {
                self.uic.background().color(self.bg_color).draw(gl);
            }
        }
    }

    pub fn render_ui(&mut self, gl: &mut Gl, uic: &mut UiContext) {
    }
}
