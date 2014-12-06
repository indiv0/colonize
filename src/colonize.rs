// External crates.
extern crate current;
extern crate event;
extern crate input;
extern crate collections;
extern crate gl;
extern crate glfw;
extern crate glfw_window;
extern crate nice_glfw;
extern crate opengl_graphics;
extern crate shader_version;
extern crate window;

// External use imports.
use current::Set;
use event::{ Event, Events, MaxFps, Ups };
use glfw_window::GlfwWindow;
use nice_glfw::WindowBuilder;
// Local imports.
use app::App;

// Modules.
mod app;

fn main() {
    // TODO: Get rid of this unwrap.
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (glfw_window, events) = WindowBuilder::new(&glfw)
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

    let mut app = App::new();
    app.load();

    for e in Events::new(window)
        .set(Ups(120))
        .set(MaxFps(10_000)) {
            use input::InputEvent::{ Press, Release };

            match e {
                Event::Update(args) =>
                    app.update(&args),
                Event::Render(args) =>
                    app.render(&args),
                Event::Input(Press(button)) =>
                    app.key_press(button),
                Event::Input(Release(button)) =>
                    app.key_release(button),
                _ => {}
            }
    }
}
