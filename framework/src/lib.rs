#![cfg_attr(feature = "nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

extern crate graphics;
extern crate piston;
extern crate serde;

pub use backend::Backend;
pub use bindings::{
    BindingsBTreeMap,
    BindingsHashMap,
    BindingMap,
    BindingStore,
    Command,
    UnwrapBindings,
};
pub use draw::Draw;
pub use scene::{BoxedScene, Scene, SceneCommand};
pub use scene_manager::SceneManager;
pub use util::RustcSerializeWrapper;

pub mod backend;
mod bindings;
pub mod draw;
mod scene;
mod scene_manager;
mod util;
