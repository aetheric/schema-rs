//! This is where the magic happens. The schemas should be downloaded and
//! compiled into rust traits and implementation structs here.

#![cfg_attr(feature = "nightly", feature(rustc_private))]

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

extern crate aster;
extern crate xml;

use std::env::{ var as env_var, VarError };
use std::fs::File;
use std::io::{ Read, Write, Error as IoError };
use std::path::Path;
use std::error::Error;
use std::fmt::{ Display, Formatter, Result as FmtResult };

use aster::AstBuilder;
use syntax::print::pprust::expr_to_string;
use xml::{ Parser, Event };

fn main() {

    let builder = AstBuilder::new();
    let mut parser = Parser::new();

    File::open("./schema.rdf").map_err(BuildError::Io)
        .and_then(file_into_string)
        .map(|content| parser.feed_str(&content))
        .unwrap();

    for event in parser {
        match event.unwrap() {
            Event::PI(info) => println!("PI:{}", info),
            Event::ElementStart(tag) => println!("<{}>", tag.name),
            Event::ElementEnd(tag) => println!("</{}>", tag.name),
            Event::Characters(chars) => println!("CHARS:{}", chars),
            Event::CDATA(cdata) => println!("<!CDATA>{}</!CDATA>", cdata),
            Event::Comment(comment) => println!("<!-- {} -->", comment)
        }
    }

    let expr = builder.expr()
        .add().u32(1).u32(2);

    env_var("OUT_DIR").map_err(BuildError::Var)
        .map(|dir| Path::new(&dir).join("/schema.rs"))
        .and_then(|path| file_from_string(&path, expr_to_string(&expr)))
        .unwrap();

}

fn file_into_string(mut file: File) -> Result<String, BuildError> {
    let mut output = String::new();
    return file
        .read_to_string(&mut output).map_err(BuildError::Io)
        .map(|len| output);
}

fn file_from_string<P: AsRef<Path>>(path: P, string: String) -> Result<(), BuildError> {
    return File::create(&path)
        .and_then(|mut file| file.write_all(string.as_bytes())).map_err(BuildError::Io);
}

#[derive(Debug)]
enum BuildError {
    Io(IoError),
    Var(VarError)
}

impl Display for BuildError {
    fn fmt(&self, fmtr: &mut Formatter) -> FmtResult {
        match *self {
            BuildError::Io(ref err)  => write!(fmtr, "IO error: {}", err),
            BuildError::Var(ref err) => write!(fmtr, "Var error: {}", err)
        }
    }
}

impl Error for BuildError {
    
    fn description(&self) -> &str {
        match *self {
            BuildError::Io(ref err)  => err.description(),
            BuildError::Var(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BuildError::Io(ref err)  => Some(err),
            BuildError::Var(ref err) => Some(err)
        }
    }

}

