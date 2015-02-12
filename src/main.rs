#![feature(core)]

extern crate "colonize-backend" as backend;
extern crate event;
extern crate gfx_voxel;
extern crate input;
extern crate noise;
extern crate quack;
extern crate tcod_window;
extern crate "colonize-utility" as utility;
extern crate window;

use game::Game;

mod game;
mod gamescene;
mod gamestate;
mod menuscene;
mod scene;
mod terrain;
mod world;

fn main() {
    let game = Game::new();
    game.run();
}
