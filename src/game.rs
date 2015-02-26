use backend::Renderer;
use event::{ Events, MaxFps, Ups };
use quack::Set;
use tcod_window::TcodWindow;
use window::WindowSettings;

use gamestate::GameState;
use menuscene::MenuScene;
use scene::{ BoxedScene, Scene };

pub struct Game {
    window: TcodWindow,
    gamestate: GameState,
    current_scene: BoxedScene,
}

impl Game {
    pub fn new() -> Game {
        let window = TcodWindow::new(
            WindowSettings {
                title: "Colonize".to_string(),
                size: [99, 61],
                fullscreen: false,
                exit_on_esc: true,
                samples: 0,
            }
        );

        Game {
            window: window,
            gamestate: GameState::new(Renderer::new()),
            current_scene: MenuScene::new(),
        }
    }

    pub fn run(mut self) {
        let event_iter = Events::new(self.window).set(Ups(180)).set(MaxFps(10_000));

        for ref e in event_iter {
            match self.current_scene.handle_event(e, &mut self.gamestate) {
                Some(scene) => self.current_scene = scene,
                None => {}
            };
        }
    }
}
