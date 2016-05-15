#[derive(Deserialize, Serialize)]
pub struct Localization {
    /// Colonize - Window title
    pub colonize_window_title: String,
    /// GameScene - Welcome text
    pub gamescene_welcome_text: String,
    /// GameScene - Debug - Cursor
    pub gamescene_debug_cursor: String,
    /// GameScene - Debug - Camera
    pub gamescene_debug_camera: String,
    /// GameScene - Debug - Chunk
    pub gamescene_debug_chunk: String,
    /// Internal - Failed to build window
    pub internal_failed_to_build_window: String,
    /// Internal - Failed to load font message
    pub internal_failed_to_load_font: String,
    /// MenuScene - Menu option - Singleplayer
    pub menuscene_singleplayer: String,
    /// MenuScene - Menu option - Options
    pub menuscene_options: String,
    /// MenuScene - Menu option - Credits
    pub menuscene_credits: String,
    /// Util - Unit - Millisecond
    pub util_unit_millisecond: String,
    /// Util - Unit - FPS
    pub util_unit_fps: String,
}

#[derive(Deserialize, Serialize)]
struct ParsedLocalization {
    colonize_window_title: Option<String>,
    gamescene_welcome_text: Option<String>,
    gamescene_debug_cursor: Option<String>,
    gamescene_debug_camera: Option<String>,
    gamescene_debug_chunk: Option<String>,
    internal_failed_to_build_window: Option<String>,
    internal_failed_to_load_font: Option<String>,
    menuscene_singleplayer: Option<String>,
    menuscene_options: Option<String>,
    menuscene_credits: Option<String>,
    util_unit_millisecond: Option<String>,
    util_unit_fps: Option<String>,
}
