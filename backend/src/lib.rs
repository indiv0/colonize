#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate cgmath;
extern crate colonize_utility as utility;

pub use tcod::{ TcodRenderer, TcodWindow };

pub use rendering::Renderer;
pub use windowing::Window;

mod tcod;

pub mod rendering;
pub mod windowing;
