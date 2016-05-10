#[cfg(feature = "with-syntex")]
mod inner {
    extern crate serde_codegen;
    extern crate syntex;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        for &(src, dst) in &[
            ("src/bindings.in.rs", "bindings.rs"),
        ] {
            let src = Path::new(src);
            let dst = Path::new(&out_dir).join(dst);

            let mut registry = syntex::Registry::new();

            serde_codegen::register(&mut registry);

            registry.expand("", &src, &dst).unwrap();
        }
    }
}

#[cfg(feature = "nightly")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
