use backend::{
    Renderer,
    TcodWindow,
    Window
};
use cgmath::{EuclideanSpace, Point2};
use piston::input::Event;
use piston::input::Event::{
    Input as InputEvent,
    Render,
    Update
};
use piston::input::keyboard::Key;
use piston::input::Button::Keyboard;
use piston::input::Input;
use piston::input::Input::{Move, Press};
use piston::input::Motion::MouseCursor;
use utility::Bounds;

use bindings::{Action, Bindings};
use camera;
use camera::{Camera, CameraAction};
use command::Command;
use gamestate::GameState;
use scene::{ Scene, BoxedScene };
use world::{ Direction, World, abs_pos_to_chunk_pos };
use worldview;

pub struct GameScene {
    // TODO: replace this with a trait object
    bindings: Bindings,
    msg_window: TcodWindow,
    world: World,
    camera: Camera,
    mouse_pos: Point2<f64>,
}

impl GameScene {
    pub fn boxed_new() -> BoxedScene {
        let bindings = Bindings::new()
            .add_binding(Key::Down, Action::Camera(CameraAction::Move(Direction::South)))
            .add_binding(Key::Less, Action::Camera(CameraAction::Move(Direction::Down)))
            .add_binding(Key::Up, Action::Camera(CameraAction::Move(Direction::North)))
            .add_binding(Key::Left, Action::Camera(CameraAction::Move(Direction::West)))
            .add_binding(Key::Right, Action::Camera(CameraAction::Move(Direction::East)))
            .add_binding(Key::Greater, Action::Camera(CameraAction::Move(Direction::Up)));

        Box::new(GameScene {
            bindings: bindings,
            msg_window: TcodWindow::new(Bounds::new(0, 54, 99, 61)),
            world: World::new(None, 3),
            camera: Camera::default(),
            mouse_pos: Point2::origin(),
        })
    }

    fn get_command_from_input<'a>(&'a mut self, input: &Input) -> Option<Command> {
        match *input {
            Press(Keyboard(key)) => {
                match self.bindings.get_action_from_binding(&key) {
                    Some(action) => {
                        match *action {
                            Action::Camera(ref action) => {
                                match *action {
                                    CameraAction::Move(ref direction) => Some(camera::new_move_camera_command(direction, &mut self.camera)),
                                }
                            },
                        }
                    }
                    _ => None,
                }
            },
            Move(MouseCursor(x, y)) => {
                self.mouse_pos = Point2::new(x, y);
                None
            },
            _ => None,
        }
    }
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        let maybe_scene = None;
        match *e {
            InputEvent(ref input) => {
                let command = self.get_command_from_input(input);
                if let Some(mut command) = command {
                    command();
                }
            },
            Update(_) => {
                let mut msg = String::new();
                msg.push_str("Welcome to Colonize!\n");
                msg.push_str(&*format!("Mouse Cursor: {:?}\n", self.mouse_pos));
                msg.push_str(&*format!("Camera: {:?}\n", self.camera.get_position()));
                msg.push_str(&*format!("Chunk: {:?}", abs_pos_to_chunk_pos(self.camera.get_position())));
                self.msg_window.flush_message_buffer();
                self.msg_window.buffer_message(&*msg);
            },
            Render(_) => {
                let renderer = state.get_renderer();

                renderer.before_render();
                renderer.attach_window(&mut self.msg_window);
                // TODO: refactor these magic numbers.
                worldview::draw_world(&self.world, renderer, Bounds::new(0, 0, 54, 49), &self.camera);
                renderer.render_frame();
            },
            _ => {}
        }
        maybe_scene
    }
}
