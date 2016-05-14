#[derive(Deserialize, Serialize)]
pub struct Localization {
    #[serde(default)]
    /// Colonize - Window title
    pub colonize_window_title: String,
    #[serde(default)]
    /// GameScene - Welcome text
    pub gamescene_welcome_text: String,
    #[serde(default)]
    /// GameScene - Debug - Cursor
    pub gamescene_debug_cursor: String,
    #[serde(default)]
    /// GameScene - Debug - Camera
    pub gamescene_debug_camera: String,
    #[serde(default)]
    /// GameScene - Debug - Chunk
    pub gamescene_debug_chunk: String,
    #[serde(default)]
    /// Internal - Failed to build window
    pub internal_failed_to_build_window: String,
    #[serde(default)]
    /// Internal - Failed to load font message
    pub internal_failed_to_load_font: String,
    #[serde(default)]
    /// MenuScene - Menu option - Singleplayer
    pub menuscene_singleplayer: String,
    #[serde(default)]
    /// MenuScene - Menu option - Options
    pub menuscene_options: String,
    #[serde(default)]
    /// MenuScene - Menu option - Credits
    pub menuscene_credits: String,
    #[serde(default)]
    /// Util - Unit - Millisecond
    pub util_unit_millisecond: String,
    #[serde(default)]
    /// Util - Unit - FPS
    pub util_unit_fps: String,
}
