#[cfg(feature = "nightly")]
include!("bindings.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
