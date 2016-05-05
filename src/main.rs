#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate colonize_backend as backend;
extern crate cgmath;
extern crate piston;
extern crate tcod;
extern crate tcod_window;
extern crate colonize_utility as utility;
extern crate colonize_world as world;

mod bindings;
mod camera;
mod command;
mod config;
mod drawable;
mod game;
mod gamescene;
mod gamestate;
mod menuscene;
mod scene;
mod worldview;

use backend::TcodRenderer;
use piston::window::{
    Size,
    WindowSettings,
};
use tcod_window::TcodWindow;

use config::Config;
use game::Game;

fn main() {
    // TODO: load this from a config file.
    // TODO: make this changeable from the options menu.
    let config = Config {
        window_height: 61,
        window_width: 99,
        window_title: "Colonize".to_owned(),
        ups: 180,
        max_fps: 10_000,
    };

    let settings = WindowSettings::new(
            config.window_title.clone(),
            Size {
                width: config.window_width,
                height: config.window_height,
            }
        )
        .exit_on_esc(true);
    let window: TcodWindow = settings.build()
        .expect("Failed to build window.");
    let renderer = TcodRenderer::new(window.window.clone());

    let mut game = Game::new(config, window, renderer);
    game.run();
}
