extern crate glium_text;

use std::fs::File;
use std::path::Path;

use cgmath::Vector2;
use glium::Surface;
use glium::backend::{Context, Facade};

use self::glium_text::{FontTexture, TextDisplay, TextSystem};

use color::Color;

pub struct TextRenderer {
    system: TextSystem,
    font: FontTexture,
    font_size: f32,
}

impl TextRenderer {
    pub fn new<F>(facade: &F, font_name: &Path, font_size: u32) -> TextRenderer
        where F: Facade,
    {
        let system = TextSystem::new(facade);
        let font_file = File::open(font_name).unwrap();
        let font = FontTexture::new(
                facade,
                &font_file,
                font_size).unwrap();

        TextRenderer {
            system: system,
            font: font,
            font_size: font_size as f32,
        }
    }

    pub fn draw<S>(
        &mut self,
        backend: &Context,
        surface: &mut S,
        text: &str,
        position:
        Vector2<f32>,
        color: Color
    )
        where S: Surface,
    {
        let (w, h) = backend.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let text_display = TextDisplay::new(&self.system, &self.font, text);
        // FIXME: make sure text renders at the correct scale.
        let _text_width = text_display.get_width();

        let scale = self.font_size / h;
        let matrix: [[f32; 4]; 4] = [
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [2.0 * (position.x / w) - 1.0, -2.0 * (position.y / h) + 1.0, 1.0, 1.0],
        ];

        glium_text::draw(&text_display, &self.system, surface, matrix, color.as_tuple());
    }
}
