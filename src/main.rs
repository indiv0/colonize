#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

extern crate cgmath;
extern crate fps_counter;
#[macro_use]
extern crate glium;
extern crate glium_graphics;
extern crate piston;
#[macro_use]
extern crate rgframework;
extern crate serde;
extern crate serde_json;
extern crate shader_version;
extern crate time;
extern crate colonize_utility as utility;
extern crate colonize_world as world;

mod action;
mod camera;
mod config;
mod game;
mod localization;
mod scene;
mod tile;

use std::error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use glium_graphics::GliumWindow as Window;
use piston::window::{
    BuildFromWindowSettings,
    Size,
    WindowSettings,
};
use rgframework::backend::graphics::RenderContext;
use rgframework::rendering::{Renderer, TextRenderer};
use rgframework::manager::Manager;
use shader_version::OpenGL;

use config::Config;
use game::Game;
use localization::Localization;

type ColonizeError = Box<error::Error>;
type ColonizeResult<T> = std::result::Result<T, ColonizeError>;

const CONFIG_PATH: &'static str = "colonize.json";
const FONT_DIR: &'static str = "fonts/";
const LOCALIZATION_DIR: &'static str = "localization/";
const TEXTURES_DIR: &'static str = "textures/";
const LOCALIZATION_FILE_EXTENSION: &'static str = "json";

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

fn main() {
    // Load the configuration from its JSON file, falling back to the default
    // configuration in the event of an error.
    let config = match read_file_to_string(Path::new(CONFIG_PATH)) {
        Ok(json) => Config::from_json(&json),
        Err(_) => Config::default(),
    };

    // Define the asset path.
    let asset_path: PathBuf = (&config.asset_path).into();

    // Load the localization file for the specified language, falling back to
    // the default localization definition in the event of an error.
    let mut localization_file = asset_path.join(LOCALIZATION_DIR)
        .join(&config.language);
    localization_file.set_extension(LOCALIZATION_FILE_EXTENSION);
    let localization = match read_file_to_string(&localization_file) {
        Ok(json) => Localization::from_json(&json),
        Err(_) => Localization::default(),
    };
    let localization = Rc::new(localization);

    // Initialize the window and graphics backend.
    let window: Window = make_window(&config, &localization);

    // TODO: change this to a debug statement.
    let hidpi_factor = window.window.borrow_mut().window.hidpi_factor();
    println!("HIDPI: {}", hidpi_factor);

    // Initialize the font rendering system.
    let text_renderer = {
        let font_path = asset_path.join(FONT_DIR).join(&config.font_file);
        TextRenderer::new(&window.context, &font_path, config.font_size)
    };

    // Create the texture manager.
    let textures_path = asset_path.join(TEXTURES_DIR);
    let texture_manager = Manager::new(window.context.clone(), textures_path);

    // Create the RenderContext.
    let mut render_context = RenderContext {
        context: window.context.clone(),
    };

    // Initialize the `Sprite` renderer.
    let sprite_renderer = Renderer::new(window.context.clone());

    // Construct the `Game` object and run the game.
    let mut game = Game::new(
        config,
        localization,
        window.context.clone(),
        window,
        sprite_renderer,
        texture_manager,
        text_renderer
    );
    game.run(&mut render_context);
}

fn read_file_to_string(path: &Path) -> ColonizeResult<String> {
    let mut file = try!(File::open(&path));
    let mut file_str = String::new();
    try!(file.read_to_string(&mut file_str));
    Ok(file_str)
}

fn make_window<W>(config: &Config, localization: &Localization) -> W
    where W: BuildFromWindowSettings,
{
    WindowSettings::new(
            localization.colonize_window_title.clone(),
            Size {
                width: config.window_width,
                height: config.window_height,
            }
        )
        .exit_on_esc(config.exit_on_esc)
        .fullscreen(config.fullscreen)
        .vsync(config.vsync)
        .opengl(OPENGL_VERSION)
        .build()
        .expect(&localization.internal_failed_to_build_window)
}
