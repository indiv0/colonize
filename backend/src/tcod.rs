extern crate tcod;

use std::rc::Rc;
use std::cell::RefCell;

use self::tcod::console;
use self::tcod::console::{
    BackgroundFlag,
    Console,
    Offscreen,
    Root,
    TextAlignment,
};

use rendering::{ Renderer, TextAlign };
use utility::{ Bounds, Point2 };
use windowing::Window;

pub struct TcodWindow {
    console: Offscreen,
    bounds: Bounds<i32>,
    messages: Vec<String>,
    max_messages: u64
}

impl Window for TcodWindow {
    fn clear(&mut self) {
        self.console.clear();
    }

    fn get_bounds(&self) -> Bounds<i32> {
        self.bounds
    }

    fn print_message(&mut self, x: i32, y: i32,
            align: TextAlign, text: &str) {
        let alignment = match align {
            TextAlign::Left => TextAlignment::Left,
            TextAlign::Center => TextAlignment::Center,
            TextAlign::Right => TextAlignment::Right,
        };
        self.console.print_ex(x as i32, y as i32, BackgroundFlag::Set,
            alignment, text);
    }

    fn buffer_message(&mut self, text: &str) {
        self.messages.insert(0, text.to_owned());
        self.messages.truncate(self.max_messages as usize);
    }

    fn flush_message_buffer(&mut self) {
        for _ in 0..self.max_messages {
            self.messages.insert(0, "".to_owned());
        }
        self.messages.truncate(self.max_messages as usize);
    }

    fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }
}

impl TcodWindow {
    pub fn new(bounds: Bounds<i32>) -> TcodWindow {
        let height = bounds.max[1] - bounds.min[1] + 1;
        let width = bounds.max[0] - bounds.min[0] + 1;
        let console = Offscreen::new(
            width as i32,
            height as i32);

        let mut messages = vec!["".to_owned()];
        for _ in 0..height {
            messages.insert(0, "".to_owned());
        }
        messages.truncate(height as usize);

        TcodWindow {
            console: console,
            bounds: bounds,
            messages: messages,
            max_messages: height as u64,
        }
    }

    fn get_console(&mut self) -> Box<&Offscreen> {
        Box::new(&self.console)
    }
}

pub struct TcodRenderer {
    pub console: Rc<RefCell<Root>>,
}

impl Renderer<TcodWindow> for TcodRenderer {
    fn before_render(&mut self) {
        self.console.borrow_mut().clear();
    }

    fn render_obj(&mut self, position: Point2<i32>, symbol: char) {
        self.console.borrow_mut().put_char(position[0], position[1],
            symbol, BackgroundFlag::Set);
    }

    fn render_frame(&mut self) {
        self.console.borrow_mut().flush();
    }

    fn attach_window(&mut self, window: &mut TcodWindow) {
        window.clear();
        let bounds = window.get_bounds();
        let messages = window.get_messages();

        for (line, message) in messages.iter().enumerate() {
            window.print_message(0, line as i32, TextAlign::Left,
                &message);
        }

        let console: Box<Console> = Box::new(*window.get_console());

        console::blit(
            &console,
            (0, 0),
            ((bounds.max[0]) + 1, (bounds.max[1]) + 1),
            &mut *self.console.borrow_mut(),
            (bounds.min[0], bounds.min[1]),
            1f32,
            1f32);
    }
}

impl TcodRenderer {
    pub fn new(console: Rc<RefCell<Root>>) -> TcodRenderer {
        TcodRenderer { console: console }
    }
}
