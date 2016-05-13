use piston::input::keyboard::Key;
use rgframework::{
    BindingsHashMap,
    BindingStore,
    RustcSerializeWrapper,
};

use action::Action;
use camera::CameraAction;
use world::Direction;

create_config! {
    language: String, "en_CA".to_owned(), "Localization language";
    asset_path: String, "./assets/".to_owned(), "Directory in which game assets are located";
    font_file: String, "NotoSans/NotoSans-Regular.ttf".to_owned(), "Path to the main font file, relative to the fonts directory.";
    window_height: u32, 800, "Height of the window";
    window_width: u32, 800, "Width of the window";
    ups: u64, 180,
        "Number of updates per second.\nThis is the fixed update rate on average over time. If the event loop lags, it will try to catch up";
    max_fps: u64, 10_000,
        "Maximum number of frames per second.\nThe frame rate can be lower because the next frame is always scheduled from the previous frame. This causes the frames to \"slip\" over time.";
    exit_on_esc: bool, true, "Terminate the program upon recieving the ESC key";
    fullscreen: bool, false, "Initialize the window in fullscreen mode";
    vsync: bool, false, "Enable vsync";
    initial_world_size: u32, 3, "Radius (in chunks) of the initially generated world";
    font_size: u32, 16, "Font size for all rendered text";
    game_scene_key_bindings: BindingsHashMap<RustcSerializeWrapper<Key>, Action>, BindingsHashMap::new()
            .add_binding(RustcSerializeWrapper::new(Key::Down), Action::Camera(CameraAction::Move(Direction::South)))
            .add_binding(RustcSerializeWrapper::new(Key::Comma), Action::Camera(CameraAction::Move(Direction::Down)))
            .add_binding(RustcSerializeWrapper::new(Key::Up), Action::Camera(CameraAction::Move(Direction::North)))
            .add_binding(RustcSerializeWrapper::new(Key::Left), Action::Camera(CameraAction::Move(Direction::West)))
            .add_binding(RustcSerializeWrapper::new(Key::Right), Action::Camera(CameraAction::Move(Direction::East)))
            .add_binding(RustcSerializeWrapper::new(Key::Period), Action::Camera(CameraAction::Move(Direction::Up))),
        "Key bindings for the main game scene";
}
