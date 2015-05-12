// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "nightly", feature(rustc_private))]

// FIXME: this is required until this issue is implmented:
// https://github.com/rust-lang/rust/issues/21757
#[cfg(feature = "nightly")]
extern crate syntax as syntax_lib;

#[cfg(feature = "syntex_syntax")]
extern crate syntex_syntax as syntex_syntax_lib;

#[cfg(feature = "nightly")]
pub mod syntax {
    pub use super::syntax_lib as syntax;

    include!("lib.rs.in");
}

#[cfg(feature = "syntex_syntax")]
pub mod syntex {
    pub use super::syntex_syntax_lib as syntax;

    include!("lib.rs.in");
}

pub use syntex::syntax::parse::token::Paren;
