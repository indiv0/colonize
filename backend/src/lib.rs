#![feature(collections)]

extern crate "colonize-utility" as utility;

pub use tcod::{ Renderer, Window };

pub use rendering::RendererTrait;
pub use windowing::WindowTrait;

mod tcod;

pub mod rendering;
pub mod windowing;
