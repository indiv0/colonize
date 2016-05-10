#[cfg(feature = "nightly")]
include!("direction.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/direction.rs"));
