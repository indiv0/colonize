use backend::{
    Renderer,
    TcodWindow,
    Window
};
use piston::input::Event;
use piston::input::Event::{
    Input,
    Render,
    Update
};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use piston::input::Input::{Move, Press};
use piston::input::Motion::MouseCursor;
use utility::{ Bounds, Point2 };

use gamestate::GameState;
use scene::{ Scene, BoxedScene };
use world::{ World, CHUNK_SIZE, abs_pos_to_chunk_pos };
use worldview;

pub struct GameScene {
    // TODO: replace this with a trait object
    msg_window: TcodWindow,
    world: World,
    height: usize,
    camera_pos: Point2<i32>,
    mouse_pos: Point2<f64>,
}

impl GameScene {
    pub fn boxed_new() -> BoxedScene {
        Box::new(GameScene {
            msg_window: TcodWindow::new(Bounds::new(0, 54, 99, 61)),
            world: World::new(None, 3),
            height: 0,
            camera_pos: [0, 0],
            mouse_pos: [0.0, 0.0],
        })
    }
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        let maybe_scene = None;
        match *e {
            Input(Press(Keyboard(key))) => {
                match key {
                    Key::Less => {
                        match self.height {
                            x if x >= 1 => self.height -= 1,
                            _ => {}
                        }
                    },
                    Key::Greater => {
                        match self.height {
                            x if (x + 1) < CHUNK_SIZE => self.height += 1,
                            _ => {}
                        }
                    },
                    Key::Up => self.camera_pos[1] -= 1,
                    Key::Down => self.camera_pos[1] += 1,
                    Key::Left => self.camera_pos[0] -= 1,
                    Key::Right => self.camera_pos[0] += 1,
                    _ => {}
                }
            },
            Input(Move(MouseCursor(x, y))) => {
                self.mouse_pos = [x, y];
            },
            Update(_) => {
                let mut msg = String::new();
                msg.push_str("Welcome to Colonize!\n");
                msg.push_str(&*format!("Height: {}\n", self.height));
                msg.push_str(&*format!("Mouse Cursor: {:?}\n", self.mouse_pos));
                msg.push_str(&*format!("Camera: {:?}\n", self.camera_pos));
                msg.push_str(&*format!("Chunk: {:?}", abs_pos_to_chunk_pos(self.camera_pos)));
                self.msg_window.flush_message_buffer();
                self.msg_window.buffer_message(&*msg);
            },
            Render(_) => {
                let renderer = state.get_renderer();

                renderer.before_render();
                renderer.attach_window(&mut self.msg_window);
                // TODO: refactor these magic numbers.
                worldview::draw_world(&self.world, renderer, Bounds::new(0, 0, 54, 49), self.camera_pos, self.height);
                renderer.render_frame();
            },
            _ => {}
        }
        maybe_scene
    }
}
