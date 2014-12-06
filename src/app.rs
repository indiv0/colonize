use conrod::{
    Background,
    Button,
    Callable,
    Color,
    Colorable,
    Drawable,
    Frameable,
    Label,
    Labelable,
    Positionable,
    Shapeable,
    UiContext,
};
use event::{RenderArgs, UpdateArgs};
use input;
use opengl_graphics::Gl;

/// The current state the game is in.
enum PlayState {
    MainMenu
}

pub struct App {
    state: PlayState,
    bg_color: Color,
    frame_width: f64,
}

impl App {
    pub fn new() -> App {
        App {
            state: PlayState::MainMenu,
            bg_color: Color::new(0.2, 0.35, 0.45, 1.0),
            frame_width: 1.0,
        }
    }

    pub fn load(&self) {
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
    }

    pub fn key_press(&mut self, _button: input::Button) {
    }

    pub fn key_release(&mut self, _button: input::Button) {
    }

    pub fn render(&mut self, _args: &RenderArgs, gl: &mut Gl, uic: &mut UiContext) {
        match self.state {
            PlayState::MainMenu => {
                // Draw a background color.
                uic.background().color(self.bg_color).draw(gl);

                // Draw a label with the game name.
                uic.label("Colonize")
                    .position(10.0, 30.0)
                    .size(48u32)
                    .color(self.bg_color.plain_contrast())
                    .draw(gl);

                uic.button(0u64)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 110.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Singleplayer")
                    .callback(|| self.bg_color = Color::random())
                    .draw(gl);
                uic.button(0u64)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 140.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Multiplayer")
                    .callback(|| self.bg_color = Color::random())
                    .draw(gl);
                uic.button(0u64)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 170.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Options")
                    .callback(|| self.bg_color = Color::random())
                    .draw(gl);
                uic.button(0u64)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 200.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Credits")
                    .callback(|| self.bg_color = Color::random())
                    .draw(gl);
            }
        }
    }
}
