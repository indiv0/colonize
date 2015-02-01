use std::cell::RefCell;

use backend::Renderer;
use event::{ Events, MaxFps, Ups };
use quack::Set;
use tcod::Console;
use tcod_window::TcodWindow;
use utility::Bounds;

use gamestate::GameState;
use menuscene::MenuScene;
use scene::{ BoxedScene, Scene };

pub struct Game<'a> {
    window: TcodWindow,
    gamestate: GameState,
    current_scene: BoxedScene,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let total_bounds = Bounds::new(0, 0, 99, 61);
        let renderer = Renderer::new(total_bounds);

        Game {
            window: TcodWindow::new(Console::Root,
                                    "Colonize".to_string(),
                                    true),
            gamestate: GameState::new(renderer),
            current_scene: MenuScene::new(),
        }
    }

    pub fn run(mut self) {
        let window = RefCell::new(self.window);
        let mut event_iter = Events::new(&window).set(Ups(180)).set(MaxFps(60));

        for ref e in event_iter {
            match self.current_scene.handle_event(e, &mut self.gamestate) {
                Some(scene) => self.current_scene = scene,
                None => {}
            };
        }
    }
}
