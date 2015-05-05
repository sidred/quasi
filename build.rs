#[cfg(feature = "quasi_codegen")]
mod inner {
    extern crate syntex;
    extern crate quasi_codegen;

    use std::env;
    use std::path::Path;

    pub fn run() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let mut registry = syntex::Registry::new();
        quasi_codegen::register(&mut registry);

        let src = Path::new("tests/test_macros.rs.in");
        let dst = Path::new(&out_dir).join("test_codegen.rs");

        registry.expand("", &src, &dst).unwrap();
    }
}

#[cfg(not(feature = "quasi_codegen"))]
mod inner {
    pub fn run() {
        panic!();
    }
}

fn main() {
    inner::run();
}
