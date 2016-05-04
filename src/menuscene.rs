use backend::{Renderer, TcodWindow, Window};
use piston::input::Event;
use piston::input::Event::{Input, Render, Update};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use piston::input::Input::Press;
use utility::Bounds;

use gamescene::GameScene;
use gamestate::GameState;
use scene::{Scene, BoxedScene};

pub struct MenuScene {
    msg_window: TcodWindow,
}

impl MenuScene {
    pub fn boxed_new() -> BoxedScene {
        Box::new(MenuScene {
            // TODO: fix these hardcodings
            msg_window: TcodWindow::new(Bounds::new(10, 54, 99, 61)),
        })
    }
}

impl Scene for MenuScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match *e {
            Input(Press(Keyboard(key))) => {
                match key {
                    Key::S => {
                        Some(GameScene::boxed_new())
                    }
                    _ => None
                }
            },
            Update(_) => {
                let mut msg = String::new();
                msg.push_str("S)ingleplayer\n");
                msg.push_str("O)ptions\n");
                msg.push_str("C)redits");
                self.msg_window.flush_message_buffer();
                self.msg_window.buffer_message(&msg);

                None
            },
            Render(_) => {
                let renderer = state.get_renderer();

                renderer.before_render();
                renderer.attach_window(&mut self.msg_window);
                renderer.render_frame();

                None
            },
            _ => None
        }
    }
}
