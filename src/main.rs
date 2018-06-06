#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate serde_json;
extern crate clap;
extern crate filebuffer;

use std::io::{self, BufWriter};
use std::fs::{self, File};
use clap::{App, Arg};
use filebuffer::FileBuffer;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!
    {
        foreign_links
        {
            Io(::std::io::Error);
            Json(::serde_json::Error);
        }
    }
}
use errors::*;

fn is_json_file(path: String) -> std::result::Result<(), String>
{
    File::open(&path).map_err(|_| String::from("file not found"))?;
    if path.ends_with(".json") {
        std::result::Result::Ok(())
    } else {
        std::result::Result::Err("file must be json".into())
    }
}

fn main() -> Result<()>
{
    let matches = App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::with_name("input")
        .help("the input.json file to use")
        .required(true)
        .validator(is_json_file))
    .get_matches();

    // Here we can call .unwrap() because the argument is required.
    let input = matches.value_of("input").unwrap();

    let metadata = fs::metadata(input)?;
    let buf = FileBuffer::open(input)?;
    buf.prefetch(0, metadata.len() as usize);

    let json: serde_json::Value = serde_json::from_slice(&buf)?;
    let writer = BufWriter::new(io::stdout());
    serde_json::to_writer_pretty(writer, &json)?;

    Ok(())
}
