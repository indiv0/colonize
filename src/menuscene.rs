use event::Event;
use event::Event::Render;

use gamescene::GameScene;
use gamestate::GameState;
use scene::{Scene, BoxedScene};

pub struct MenuScene;

impl MenuScene {
    pub fn new() -> BoxedScene {
        Box::new(MenuScene)
    }
}

impl Scene for MenuScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Render(args) => {
                let mut maybe_scene = None;
                /*let (uic, gl) = state.get_drawables();
                gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                    // Draw a background color.
                    uic.background().color(self.bg_color).draw(gl);

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
                        .frame(1.0)
                        .label("Singleplayer")
                        .callback(Box::new(|| maybe_scene = Some(GameScene::new())))
                        .draw(gl);
                    const MULTIPLAYER: u64 = SINGLEPLAYER + 1;
                    uic.button(MULTIPLAYER)
                        .dimensions(90.0, 60.0)
                        .position(50.0, 150.0)
                        .rgba(0.4, 0.75, 0.6, 1.0)
                        .frame(1.0)
                        .label("Multiplayer")
                        .callback(Box::new(|| println!("TEST")))
                        .draw(gl);
                    const OPTIONS: u64 = MULTIPLAYER + 1;
                    uic.button(OPTIONS)
                        .dimensions(90.0, 60.0)
                        .position(50.0, 250.0)
                        .rgba(0.4, 0.75, 0.6, 1.0)
                        .frame(1.0)
                        .label("Options")
                        .callback(Box::new(|| self.bg_color = Color::random()))
                        .draw(gl);
                    const CREDITS: u64 = OPTIONS + 1;
                    uic.button(CREDITS)
                        .dimensions(90.0, 60.0)
                        .position(50.0, 350.0)
                        .rgba(0.4, 0.75, 0.6, 1.0)
                        .frame(1.0)
                        .label("Credits")
                        .callback(Box::new(|| self.bg_color = Color::random()))
                        .draw(gl);
                });*/

                maybe_scene
            },
            _ => None
        }
    }
}
