use piston::input::keyboard::Key;
use rgframework::{
    BindingsHashMap,
    BindingStore,
    RustcSerializeWrapper,
};

use action::Action;
use camera::CameraAction;
use world::Direction;

#[cfg(feature = "nightly")]
include!("config.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/config.rs"));

create_type_parsing_impls! {
    Config,
    ParsedConfig,
    language: String, "en_CA".to_owned();
    asset_path: String, "./assets/".to_owned();
    font_file: String, "NotoSans/NotoSans-Regular.ttf".to_owned();
    window_height: u32, 800;
    window_width: u32, 800;
    ups: u64, 180;
    max_fps: u64, 10_000;
    exit_on_esc: bool, true;
    fullscreen: bool, false;
    vsync: bool, false;
    initial_world_size: u32, 3;
    font_size: u32, 16;
    game_scene_key_bindings: BindingsHashMap<RustcSerializeWrapper<Key>, Action>, BindingsHashMap::new()
            .add_binding(RustcSerializeWrapper::new(Key::Down), Action::Camera(CameraAction::Move(Direction::South)))
            .add_binding(RustcSerializeWrapper::new(Key::Comma), Action::Camera(CameraAction::Move(Direction::Down)))
            .add_binding(RustcSerializeWrapper::new(Key::Up), Action::Camera(CameraAction::Move(Direction::North)))
            .add_binding(RustcSerializeWrapper::new(Key::Left), Action::Camera(CameraAction::Move(Direction::West)))
            .add_binding(RustcSerializeWrapper::new(Key::Right), Action::Camera(CameraAction::Move(Direction::East)))
            .add_binding(RustcSerializeWrapper::new(Key::Period), Action::Camera(CameraAction::Move(Direction::Up)));
}
