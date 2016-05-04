use backend::TcodRenderer;
use piston::event_loop::{
    EventLoop,
    Events,
    WindowEvents,
};
use piston::input::Input;
use piston::window::Window;

use config::Config;
use gamestate::GameState;
use menuscene::MenuScene;
use scene::{ BoxedScene, Scene };

pub struct Game<W>
    where W: Window<Event=Input>
{
    gamestate: GameState,
    current_scene: BoxedScene,
    events: WindowEvents,
    window: W,
}

impl<W> Game<W>
    where W: Window<Event=Input>
{
    pub fn new(config: Config, window: W, renderer: TcodRenderer) -> Game<W> {
        Game {
            events: window.events().ups(config.ups).max_fps(config.max_fps),
            gamestate: GameState::new(renderer),
            current_scene: MenuScene::boxed_new(),
            window: window,
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(scene) = self.current_scene.handle_event(&e, &mut self.gamestate) {
                self.current_scene = scene;
            }
        }
    }
}
