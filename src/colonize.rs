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
extern crate shader_version;

// External use imports.
use conrod::{Theme, UiContext};
use event::{ Event, Events, MaxFps, Ups };
use glfw_window::GlfwWindow;
use input::Input::{ Press, Release };
use nice_glfw::WindowBuilder;
use opengl_graphics::Gl;
use opengl_graphics::glyph_cache::GlyphCache;
use quack::Set;
use shader_version::opengl::OpenGL;

// Local imports.
use app::App;

// Modules.
mod app;

fn main() {
    let opengl = OpenGL::_3_0;

    // TODO: Get rid of this unwrap.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (glfw_window, events) = WindowBuilder::new(&mut glfw)
        .try_modern_context_hints()
        .size(854, 480)
        .create().expect("Couldn't create window :(");

    // Construct a window.
    let window = GlfwWindow::from_pieces(
        glfw_window,
        glfw,
        events,
        true);
    gl::load_with(|p| glfw.get_proc_address_raw(p));

    let mut event_iter = Events::new(window).set(Ups(120)).set(MaxFps(10_000));
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
                    app.render(&args, gl, &mut uic);
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
