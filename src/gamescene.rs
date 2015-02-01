use backend::{ RendererTrait, Window, WindowTrait };
use event::Event;
use event::Event::{ Input, Render, Update };
use input::keyboard::Key;
use input::Button::Keyboard;
use input::Input::Press;
use utility::Bounds;

use gamestate::GameState;
use map::Map;
use scene::{Scene, BoxedScene};

pub struct GameScene {
    msg_window: Window,
    map: Map,
    height: usize,
}

impl GameScene {
    pub fn new() -> BoxedScene {
        let mut map = Map::new();
        let chunk = map.generate_chunk();
        map.add_chunk(0, 0, chunk);

        Box::new(GameScene {
            msg_window: Window::new(Bounds::new(0, 54, 99, 61)),
            map: map,
            height: 0,
        })
    }
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Input(Press(Keyboard(key))) => {
                match key {
                    Key::Less => {
                        self.height -= 1;
                        None
                    }
                    Key::Greater => {
                        self.height += 1;
                        None
                    }
                    _ => None
                }
            },
            &Update(_) => {
                let mut msg = String::new();
                msg.push_str("Welcome to Colonize!\n");
                msg.push_str(&*format!("Height: {}", self.height));
                self.msg_window.flush_message_buffer();
                self.msg_window.buffer_message(&*msg);

                None
            },
            &Render(_) => {
                let renderer = state.get_renderer();

                renderer.before_render();
                renderer.attach_window(&mut self.msg_window);
                self.map.render(renderer, Bounds::new(0, 0, 78, 49), self.height);
                renderer.render_frame();

                None
            },
            _ => None
        }
    }
}
