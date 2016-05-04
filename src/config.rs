pub struct Config {
    /// The height of the window.
    pub window_height: u32,
    /// The width of the window.
    pub window_width: u32,
    /// The title of the window.
    pub window_title: String,
    /// The number of updates per second.
    /// This is the fixed update rate on average over time. If the event loop
    /// lags, it will try to catch up.
    pub ups: u64,
    /// The maximum number of frames per second.
    /// The frame rate can be lower because the next frame is always scheduled
    /// from the previous frame. This causes the frames to "slip" over time.
    pub max_fps: u64,
}
