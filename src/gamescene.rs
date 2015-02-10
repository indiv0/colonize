use backend::{ RendererTrait, Window, WindowTrait };
use event::Event;
use event::Event::{ Input, Render, Update };
use input::keyboard::Key;
use input::Button::Keyboard;
use input::Input::Press;
use utility::{ Bounds, Point };

use chunk;
use gamestate::GameState;
use world::World;
use scene::{Scene, BoxedScene};

const INITIAL_SIZE: i32 = 3;

pub struct GameScene {
    msg_window: Window,
    world: World,
    height: usize,
    camera_pos: Point,
}

impl GameScene {
    pub fn new() -> BoxedScene {
        let mut world = World::new();

        for x in (-INITIAL_SIZE .. INITIAL_SIZE) {
            for y in (-INITIAL_SIZE .. INITIAL_SIZE) {
                let chunk = world.generate_chunk(x, y);
                world.add_chunk(x, y, chunk);
            }
        }

        Box::new(GameScene {
            msg_window: Window::new(Bounds::new(0, 54, 99, 61)),
            world: world,
            height: 0,
            camera_pos: Point { x: 0, y: 0 },
        })
    }
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        let maybe_scene = None;
        match e {
            &Input(Press(Keyboard(key))) => {
                match key {
                    Key::Less => {
                        match self.height {
                            x if x >= 1 => self.height -= 1,
                            _ => {}
                        }
                    },
                    Key::Greater => {
                        match self.height {
                            x if (x + 1) < chunk::SIZE => self.height += 1,
                            _ => {}
                        }
                    },
                    Key::Up => self.camera_pos.y += 1,
                    Key::Down => self.camera_pos.y -= 1,
                    Key::Left => self.camera_pos.x += 1,
                    Key::Right => self.camera_pos.x -= 1,
                    _ => {}
                }
            },
            &Update(_) => {
                let mut msg = String::new();
                msg.push_str("Welcome to Colonize!\n");
                msg.push_str(&*format!("Height: {}", self.height));
                self.msg_window.flush_message_buffer();
                self.msg_window.buffer_message(&*msg);
            },
            &Render(_) => {
                let renderer = state.get_renderer();

                renderer.before_render();
                renderer.attach_window(&mut self.msg_window);
                self.world.render(renderer, Bounds::new(0, 0, 54, 49), self.camera_pos, self.height);
                renderer.render_frame();
            },
            _ => {}
        }
        maybe_scene
    }
}
