#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
#![feature(slice_patterns)]

extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;
extern crate piston;
extern crate rand;
extern crate serde;
extern crate time;

pub use bindings::{
    BindingsBTreeMap,
    BindingsHashMap,
    BindingMap,
    BindingStore,
    Command,
    UnwrapBindings,
};
pub use id::Id;
pub use scene::{BoxedScene, Scene, SceneCommand};
pub use scene_manager::SceneManager;
pub use util::RustcSerializeWrapper;

pub mod backend;
mod bindings;
pub mod color;
pub mod id;
mod macros;
pub mod manager;
pub mod rectangle;
pub mod rendering;
mod scene;
mod scene_manager;
pub mod texture;
mod transform;
mod util;
