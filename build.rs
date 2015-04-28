extern crate syntex;
extern crate quasi_macros;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut registry = syntex::Registry::new();
    quasi_macros::register(&mut registry);

    let src = Path::new("tests/test.rss").unwrap();
    let dst = Path::new(out_dir).join("test.rs");

    registry.expand(src, dst).unwrap();
}
