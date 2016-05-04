use std::collections::HashMap;

use piston::input::keyboard::Key;

use camera::CameraAction;

pub type Binding = Key;

pub enum Action {
    Camera(CameraAction),
}

pub struct Bindings {
    bindings: HashMap<Binding, Action>,
}

impl Bindings {
    pub fn new() -> Self {
        Bindings {
            bindings: HashMap::new(),
        }
    }

    pub fn add_binding(mut self, binding: Binding, action: Action) -> Self {
        self.bindings.insert(binding, action);
        self
    }

    pub fn get_action_from_binding(&self, binding: &Binding) -> Option<&Action> {
        self.bindings.get(&binding)
    }
}

impl Default for Bindings {
    fn default() -> Self {
        Bindings::new()
    }
}
