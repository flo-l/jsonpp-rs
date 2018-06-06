#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate serde_json;
extern crate serde_transcode;
extern crate clap;
extern crate filebuffer;

use std::io::{self, BufReader, BufWriter, Read};
use std::fs::{self, File};
use std::ffi::{OsStr, OsString};
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

fn is_file(path: &OsStr) -> std::result::Result<(), OsString>
{
    File::open(&path).map_err(|_| OsString::from("file not found")).and(Ok(()))
}

fn prettify<S: Read>(source: S) -> Result<()>
{
    let writer = BufWriter::new(io::stdout());

    let mut deserializer = serde_json::Deserializer::from_reader(source);
    let mut serializer = serde_json::Serializer::pretty(writer);
    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(())
}

fn main() -> Result<()>
{
    let matches = App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::with_name("input")
        .help("the input.json file to use")
        .validator_os(is_file))
    .get_matches();

    if let Some(input) = matches.value_of("input") {
        let metadata = fs::metadata(input)?;
        let buf = FileBuffer::open(input)?;
        buf.prefetch(0, metadata.len() as usize);
        prettify(&*buf)
    } else {
        prettify(BufReader::new(io::stdin()))
    }
}
