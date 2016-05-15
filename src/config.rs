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
    language, "en_CA".to_owned();
    asset_path, "./assets/".to_owned();
    font_file, "NotoSans/NotoSans-Regular.ttf".to_owned();
    window_height, 800;
    window_width, 800;
    ups, 180;
    max_fps, 10_000;
    exit_on_esc, true;
    fullscreen, false;
    vsync, false;
    initial_world_size, 3;
    font_size, 16;
    game_scene_key_bindings, BindingsHashMap::new()
            .add_binding(RustcSerializeWrapper::new(Key::Down), Action::Camera(CameraAction::Move(Direction::South)))
            .add_binding(RustcSerializeWrapper::new(Key::Comma), Action::Camera(CameraAction::Move(Direction::Down)))
            .add_binding(RustcSerializeWrapper::new(Key::Up), Action::Camera(CameraAction::Move(Direction::North)))
            .add_binding(RustcSerializeWrapper::new(Key::Left), Action::Camera(CameraAction::Move(Direction::West)))
            .add_binding(RustcSerializeWrapper::new(Key::Right), Action::Camera(CameraAction::Move(Direction::East)))
            .add_binding(RustcSerializeWrapper::new(Key::Period), Action::Camera(CameraAction::Move(Direction::Up)));
}
