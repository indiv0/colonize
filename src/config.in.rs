#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Localization language
    pub language: String,
    /// Directory in which game assets are located
    pub asset_path: String,
    /// Path to the main font file, relative to the fonts directory.
    pub font_file: String,
    /// Height of the window
    pub window_height: u32,
    /// Width of the window
    pub window_width: u32,
    /// Number of updates per second.
    /// This is the fixed update rate on average over time. If the event loop
    /// lags, it will try to catch up.
    pub ups: u64,
    /// Maximum number of frames per second.
    /// The frame rate can be lower because the next frame is always scheduled
    /// from the previous frame. This causes the frames to "slip" over time.
    pub max_fps: u64,
    /// Terminate the program upon recieving the ESC key
    pub exit_on_esc: bool,
    /// Initialize the window in fullscreen mode
    pub fullscreen: bool,
    /// Enable vsync
    pub vsync: bool,
    /// Radius (in chunks) of the initially generated world
    pub initial_world_size: u32,
    /// Font size for all rendered text
    pub font_size: u32,
    /// Key bindings for the main game scene
    pub game_scene_key_bindings: BindingsHashMap<RustcSerializeWrapper<Key>, Action>,
}

#[derive(Deserialize, Serialize)]
struct ParsedConfig {
    language: Option<String>,
    asset_path: Option<String>,
    font_file: Option<String>,
    window_height: Option<u32>,
    window_width: Option<u32>,
    ups: Option<u64>,
    max_fps: Option<u64>,
    exit_on_esc: Option<bool>,
    fullscreen: Option<bool>,
    vsync: Option<bool>,
    initial_world_size: Option<u32>,
    font_size: Option<u32>,
    game_scene_key_bindings: Option<BindingsHashMap<RustcSerializeWrapper<Key>, Action>>,
}
