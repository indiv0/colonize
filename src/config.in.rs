#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    /// Localization language
    pub language: String,
    #[serde(default)]
    /// Directory in which game assets are located
    pub asset_path: String,
    #[serde(default)]
    /// Path to the main font file, relative to the fonts directory.
    pub font_file: String,
    #[serde(default)]
    /// Height of the window
    pub window_height: u32,
    #[serde(default)]
    /// Width of the window
    pub window_width: u32,
    #[serde(default)]
    /// Number of updates per second.
    /// This is the fixed update rate on average over time. If the event loop
    /// lags, it will try to catch up.
    pub ups: u64,
    #[serde(default)]
    /// Maximum number of frames per second.
    /// The frame rate can be lower because the next frame is always scheduled
    /// from the previous frame. This causes the frames to "slip" over time.
    pub max_fps: u64,
    #[serde(default)]
    /// Terminate the program upon recieving the ESC key
    pub exit_on_esc: bool,
    #[serde(default)]
    /// Initialize the window in fullscreen mode
    pub fullscreen: bool,
    #[serde(default)]
    /// Enable vsync
    pub vsync: bool,
    #[serde(default)]
    /// Radius (in chunks) of the initially generated world
    pub initial_world_size: u32,
    #[serde(default)]
    /// Font size for all rendered text
    pub font_size: u32,
    #[serde(default)]
    /// Key bindings for the main game scene
    pub game_scene_key_bindings: BindingsHashMap<RustcSerializeWrapper<Key>, Action>,
}
