#[cfg(feature = "nightly")]
include!("localization.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/localization.rs"));

create_type_parsing_impls! {
    Localization,
    ParsedLocalization,
    colonize_window_title: String, "Colonize".to_owned();
    gamescene_welcome_text: String, "Welcome to Colonize!".to_owned();
    gamescene_debug_cursor: String, "Mouse Cursor".to_owned();
    gamescene_debug_camera: String, "Camera".to_owned();
    gamescene_debug_chunk: String, "Chunk".to_owned();
    internal_failed_to_build_window: String, "Failed to build window".to_owned();
    internal_failed_to_load_font: String, "Failed to load font".to_owned();
    menuscene_singleplayer: String, "S)ingleplayer".to_owned();
    menuscene_options: String, "O)ptions".to_owned();
    menuscene_credits: String, "C)redits".to_owned();
    util_unit_millisecond: String, "ms".to_owned();
    util_unit_fps: String, "FPS".to_owned();
}
