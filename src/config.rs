#[cfg(feature = "nightly")]
include!("config.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/config.rs"));
