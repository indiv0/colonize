extern crate tcod;
use self::tcod::{ BackgroundFlag, Console, TextAlignment };

use rendering::{ RendererTrait, TextAlign };
use utility::{ Bounds, Point };
use windowing::WindowTrait;

pub struct Window {
    console: Console,
    bounds: Bounds,
    messages: Vec<Box<String>>,
    max_messages: u64
}

impl WindowTrait for Window {
    fn clear(&mut self) {
        self.console.clear();
    }

    fn get_bounds(&self) -> Bounds {
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
        self.messages.insert(0, Box::new(String::from_str(text)));
        self.messages.truncate(self.max_messages as usize);
    }

    fn flush_message_buffer(&mut self) {
        for _ in 0..self.max_messages {
            self.messages.insert(0, Box::new(String::from_str("")));
        }
        self.messages.truncate(self.max_messages as usize);
    }

    fn get_messages(&self) -> Vec<Box<String>> {
        self.messages.clone()
    }
}

impl Window {
    pub fn new(bounds: Bounds) -> Window {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Console::new(width as i32, height as i32);

        let mut messages = vec![Box::new(String::from_str(""))];
        for _ in (0..height) {
            messages.insert(0, Box::new(String::from_str("")));
        }
        messages.truncate(height as usize);

        Window {
            console: console,
            bounds: bounds,
            messages: messages,
            max_messages: height as u64,
        }
    }

    fn get_console(&mut self) -> &mut Console {
        &mut self.console
    }
}

pub struct Renderer {
    console: Console,
}

impl RendererTrait<Window> for Renderer {
    fn before_render(&mut self) {
        self.console.clear();
    }

    fn render_obj(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as i32, position.y as i32,
            symbol, BackgroundFlag::Set);
    }

    fn render_frame(&mut self) {
        Console::flush();
    }

    fn attach_window(&mut self, window: &mut Window) {
        window.clear();
        let bounds = window.get_bounds();
        let mut line = 0i32;
        let messages = window.get_messages();

        for message in messages.iter() {
            window.print_message(0, line, TextAlign::Left,
                &message[]);
            line += 1;
        }

        let console = window.get_console();

        Console::blit(&*console, 0, 0, (bounds.max.x as i32) + 1,
            (bounds.max.y as i32) + 1, &mut self.console,
            bounds.min.x as i32, bounds.min.y as i32,
            1f32, 1f32);
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { console: Console::Root }
    }

    pub fn get_console(&self) -> Console {
        Console::Root
    }
}
