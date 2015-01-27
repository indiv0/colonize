extern crate conrod;
extern crate event;
extern crate input;
extern crate gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate quack;
extern crate sdl2_window;
extern crate shader_version;

use std::cell::RefCell;

use conrod::{Theme, UiContext};
use event::{
    Events,
    MaxFps,
    Ups,
    WindowSettings,
};
use opengl_graphics::Gl;
use opengl_graphics::glyph_cache::GlyphCache;
use quack::Set;
use sdl2_window::Sdl2Window;
use shader_version::opengl::OpenGL;

// Local imports.
use scene::Scene;

// Modules.
mod gamescene;
mod gamestate;
mod grid;
mod menuscene;
mod scene;

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        WindowSettings {
            title: "Colonize".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );
    let window = RefCell::new(window);
    let mut event_iter = Events::new(&window).set(Ups(180)).set(MaxFps(60));

    let asset_path = Path::new("./assets");

    // Load font and generate UiContext/
    let font_path = asset_path.join("Cyanotype.ttf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let context = UiContext::new(glyph_cache, theme);

    let mut gamestate = gamestate::GameState::new(context, Gl::new(opengl), &asset_path);
    let mut current_scene = menuscene::MenuScene::new();

    for ref e in event_iter {
        gamestate.get_uic().handle_event(e);
        match current_scene.handle_event(e, &mut gamestate) {
            Some(scene) => current_scene = scene,
            None => ()
        };
    }
}
