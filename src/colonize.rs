// External crates.
extern crate conrod;
extern crate event;
extern crate input;
extern crate gl;
extern crate glfw;
extern crate glfw_window;
extern crate nice_glfw;
extern crate opengl_graphics;
extern crate quack;
extern crate sdl2_window;
extern crate shader_version;

use std::cell::RefCell;

// External use imports.
use conrod::{Theme, UiContext};
use event::{
    Event,
    Events,
    MaxFps,
    Ups,
    WindowSettings,
};
use input::Input::{ Press, Release };
use opengl_graphics::Gl;
use opengl_graphics::glyph_cache::GlyphCache;
use quack::Set;
use sdl2_window::Sdl2Window;
use shader_version::opengl::OpenGL;

// Local imports.
use app::App;

// Modules.
mod app;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Colonize".to_string(),
            size: [1180, 580],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );
    let window = RefCell::new(window);
    let mut event_iter = Events::new(&window).set(Ups(180)).set(MaxFps(60));
    let mut gl = Gl::new(opengl);

    // Load font and generate UiContext/
    let font_path = Path::new("./assets/Cyanotype.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut uic = UiContext::new(glyph_cache, theme);

    let mut app = App::new();
    app.load();

    for e in event_iter {
        uic.handle_event(&e);
        match e {
            Event::Update(args) =>
                app.update(&args),
            Event::Render(args) => {
                gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                    app.render(gl, &mut uic);
                });
            },
            Event::Input(Press(button)) =>
                app.key_press(button),
            Event::Input(Release(button)) =>
                app.key_release(button),
            _ => {}
        }
    }
}
