#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

extern crate cgmath;
extern crate fps_counter;
extern crate glium_graphics;
extern crate graphics;
extern crate opengl_graphics;
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
mod backend;
mod camera;
mod config;
mod game;
mod localization;
mod scene;
mod textures;

use std::error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use glium_graphics::GliumWindow as Window;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::window::{
    BuildFromWindowSettings,
    Size,
    WindowSettings,
};
use shader_version::OpenGL;

use config::Config;
use localization::Localization;
use game::Game;

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
    let config: Config = deserialize_from_file_or_default(&CONFIG_PATH.into());

    // Define the asset path.
    let asset_path: PathBuf = (&config.asset_path).into();

    // Load the localization file for the specified language, falling back to
    // the default localization definition in the event of an error.
    let mut localization_file = asset_path.join(LOCALIZATION_DIR)
        .join(&config.language);
    localization_file.set_extension(LOCALIZATION_FILE_EXTENSION);
    let localization: Localization = deserialize_from_file_or_default(&localization_file);

    // Initialize the window and graphics backend.
    let window: Window = make_window(&config, &localization);
    let mut gl = GlGraphics::new(OPENGL_VERSION);

    // Initialize the glyph cache.
    let mut glyph_cache = GlyphCache::new(&asset_path.join(FONT_DIR).join(&config.font_file))
        .expect(&localization.internal_failed_to_load_font);

    // Load all required textures.
    let textures_path = asset_path.join(TEXTURES_DIR);
    let textures = textures::load_textures_opengl(&textures_path);

    // Construct the `Game` object and run the game.
    let mut game = Game::new(config, localization, window, textures);
    game.run(&mut gl, &mut glyph_cache);
}

fn deserialize_from_file_or_default<T>(path: &PathBuf) -> T
    where T: Default + serde::de::Deserialize,
{
    match read_file_to_string(path){
        Ok(json) => serde_json::from_str(&json).ok().unwrap_or_default(),
        Err(_) => T::default(),
    }
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
