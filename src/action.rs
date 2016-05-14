use camera::CameraAction;

#[cfg(feature = "nightly")]
include!("action.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/action.rs"));
