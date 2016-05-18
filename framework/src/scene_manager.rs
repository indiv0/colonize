use glium::Surface;
use piston::input::GenericEvent;

use scene::{BoxedScene, SceneCommand};

pub struct SceneManager<E, S>
    where E: GenericEvent,
{
    scene_stack: Vec<BoxedScene<E, S>>,
}

impl<E, S> SceneManager<E, S>
    where E: GenericEvent,
          S: Surface,
{
    pub fn new() -> Self {
        SceneManager::default()
    }

    pub fn scene_count(&self) -> usize {
        self.scene_stack.len()
    }

    pub fn set_scene(&mut self, scene: BoxedScene<E, S>) {
        self.scene_stack.pop();
        self.scene_stack.push(scene);
    }

    pub fn push_scene(&mut self, scene: BoxedScene<E, S>) {
        self.scene_stack.push(scene);
    }

    pub fn pop_scene(&mut self) -> Option<BoxedScene<E, S>> {
        self.scene_stack.pop()
    }

    pub fn clear(&mut self) {
        self.scene_stack.clear()
    }

    pub fn handle_event(&mut self, e: &E) {
        if let Some(mut scene) = self.scene_stack.pop() {
            let result = scene.handle_event(e);
            self.scene_stack.push(scene);
            self.handle_scene_command(result);
        }
    }

    pub fn handle_scene_command(&mut self, command: Option<SceneCommand<E, S>>) {
        if let Some(command) = command {
            match command {
                SceneCommand::SetScene(scene) => {
                    self.set_scene(scene);
                },
                SceneCommand::PushScene(scene) => {
                    self.push_scene(scene);
                },
                SceneCommand::PopScene => {
                    self.pop_scene();
                },
                SceneCommand::Clear => {
                    self.clear();
                },
            }
        }
    }
}

impl<E, S> Default for SceneManager<E, S>
    where E: GenericEvent,
{
    fn default() -> Self {
        SceneManager {
            scene_stack: Vec::new(),
        }
    }
}
