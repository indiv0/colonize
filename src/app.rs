use event::{RenderArgs, UpdateArgs};
use input::Button;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn load(&self) {
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
    }

    pub fn key_press(&mut self, _button: Button) {
    }

    pub fn key_release(&mut self, _button: Button) {
    }

    pub fn render(&mut self, _args: &RenderArgs) {
    }
}
