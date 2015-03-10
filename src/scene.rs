use piston::input::Event;

use gamestate::GameState;

pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene>;
}
