#[cfg(feature = "nightly")]
include!("localization.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/localization.rs"));
