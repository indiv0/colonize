#[cfg(feature = "nightly")]
include!("localization.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/localization.rs"));

create_type_parsing_impls! {
    Localization,
    ParsedLocalization,
    colonize_window_title, "Colonize".to_owned();
    debug_render_info, "Render Info".to_owned();
    gamescene_welcome_text, "Welcome to Colonize!".to_owned();
    gamescene_debug_cursor, "Mouse Cursor".to_owned();
    gamescene_debug_camera, "Camera".to_owned();
    gamescene_debug_chunk, "Chunk".to_owned();
    internal_failed_to_build_window, "Failed to build window".to_owned();
    internal_failed_to_load_font, "Failed to load font".to_owned();
    menuscene_singleplayer, "S)ingleplayer".to_owned();
    menuscene_options, "O)ptions".to_owned();
    menuscene_credits, "C)redits".to_owned();
    util_unit_millisecond, "ms".to_owned();
    util_unit_fps, "FPS".to_owned();
}
