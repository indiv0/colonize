use backend::{RendererTrait, Window, WindowTrait};
use event::Event;
use event::Event::{Input, Render, Update};
use input::keyboard::Key;
use input::Button::Keyboard;
use input::Input::Press;
use utility::Bounds;

use gamescene::GameScene;
use gamestate::GameState;
use scene::{Scene, BoxedScene};

pub struct MenuScene {
    msg_window: Window,
}

impl MenuScene {
    pub fn new() -> BoxedScene {
        Box::new(MenuScene {
            msg_window: Window::new(Bounds::new(10, 54, 99, 61)),
        })
    }
}

impl Scene for MenuScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Input(Press(Keyboard(key))) => {
                match key {
                    Key::S => {
                        Some(GameScene::new())
                    }
                    _ => None
                }
            },
            &Update(_) => {
                let mut msg = String::new();
                msg.push_str("SINGLEPLAYER\n");
                msg.push_str("OPTIONS\n");
                msg.push_str("CREDITS");
                self.msg_window.flush_message_buffer();
                self.msg_window.buffer_message(msg.as_slice());

                None
            },
            &Render(_) => {
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
