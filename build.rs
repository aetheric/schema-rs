//! This is where the magic happens. The schemas should be downloaded and
//! compiled into rust traits and implementation structs here.

#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use aster::AstBuilder;
use syntax::print::pprust::expr_to_string;

fn main() {

    let builder = AstBuilder::new();

    // read the source schema into expressions.

    let expr = builder.expr()
        .add().u32(1).u32(2);

    let dest = env::var("OUT_DIR").unwrap();
    let path = Path::new(&dest).join("schema.rs");
    let mut file = File::create(&path).unwrap();

    file.write_all(expr_to_string(&expr).as_bytes());

}
