// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "nightly", feature(rustc_private))]

#[macro_use]
mod codegen_macro;

#[cfg(feature = "nightly")]
pub mod syntax {
    extern crate aster as aster_;
    extern crate syntax;

    use self::aster_::syntax as aster;

    module!(["quasi", "syntax"]);
}

#[cfg(feature = "syntex")]
pub mod syntex {
    module!(["quasi", "syntex"]);

    pub fn register(reg: &mut syntex::Registry) {
        reg.register_macro("quote_tokens", expand_quote_tokens);
        reg.register_macro("quote_ty", expand_quote_ty);
        reg.register_macro("quote_expr", expand_quote_expr);
        reg.register_macro("quote_matcher", expand_quote_matcher);
        reg.register_macro("quote_stmt", expand_quote_stmt);
        reg.register_macro("quote_attr", expand_quote_attr);
        reg.register_macro("quote_pat", expand_quote_pat);
        reg.register_macro("quote_arm", expand_quote_arm);
        reg.register_macro("quote_block", expand_quote_block);
        reg.register_macro("quote_item", expand_quote_item);
        reg.register_macro("quote_impl_item", expand_quote_impl_item);
        //reg.register_macro("quote_where_clause", expand_quote_where_clause);
    }
}
