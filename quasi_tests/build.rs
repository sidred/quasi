#[cfg(not(feature = "nightly"))]
extern crate syntex;

#[cfg(not(feature = "nightly"))]
extern crate quasi_codegen;

#[cfg(feature = "nightly")]
mod inner {
    pub fn main() {}
}

#[cfg(not(feature = "nightly"))]
mod inner {
    use syntex;
    use quasi_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let mut registry = syntex::Registry::new();
        quasi_codegen::register(&mut registry);

        let src = Path::new("tests/test.rs.in");
        let dst = Path::new(&out_dir).join("test.rs");

        registry.expand("", &src, &dst).unwrap();
    }
}

pub fn main() {
    inner::main();
}
