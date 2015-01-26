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
use event::UpdateArgs;
use input;
use opengl_graphics::Gl;

/// The current state the game is in.
enum PlayState {
    MainMenu,
    SinglePlayer
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

    pub fn draw_ui(&mut self, gl: &mut Gl, uic: &mut UiContext) {
        // Draw a background color.
        uic.background().color(self.bg_color).draw(gl);

        match self.state {
            PlayState::MainMenu => {
                // Draw a label with the game name.
                uic.label("Colonize")
                    .position(10.0, 10.0)
                    .size(48u32)
                    .color(self.bg_color.plain_contrast())
                    .draw(gl);

                const SINGLEPLAYER: u64 = 1;
                uic.button(SINGLEPLAYER)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 50.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Singleplayer")
                    .callback(Box::new(|| self.state = PlayState::SinglePlayer))
                    .draw(gl);
                const MULTIPLAYER: u64 = SINGLEPLAYER + 1;
                uic.button(MULTIPLAYER)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 150.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Multiplayer")
                    .callback(Box::new(|| println!("TEST")))
                    .draw(gl);
                const OPTIONS: u64 = MULTIPLAYER + 1;
                uic.button(OPTIONS)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 250.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Options")
                    .callback(Box::new(|| self.bg_color = Color::random()))
                    .draw(gl);
                const CREDITS: u64 = OPTIONS + 1;
                uic.button(CREDITS)
                    .dimensions(90.0, 60.0)
                    .position(50.0, 350.0)
                    .rgba(0.4, 0.75, 0.6, 1.0)
                    .frame(self.frame_width)
                    .label("Credits")
                    .callback(Box::new(|| self.bg_color = Color::random()))
                    .draw(gl);
            },
            PlayState::SinglePlayer => {
                uic.label("SP")
                    .position(10.0, 30.0)
                    .size(48u32)
                    .color(self.bg_color.plain_contrast())
                    .draw(gl);
            },
        }
    }
}
