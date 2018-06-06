#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_json;
extern crate itertools;
extern crate clap;
extern crate boolinator;
extern crate csv;

use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;
use clap::{App, Arg};
use itertools::Itertools;
use boolinator::Boolinator;

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

type JsonMap = serde_json::Map<String, serde_json::Value>;

fn parse_file<P: AsRef<Path>>(path: P) -> Result<serde_json::Value>
{
    // open file and read content
    let mut f = File::open(path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;

    // parse the string of data into serde_json::Value and return it
    let v: serde_json::Value = serde_json::from_str(&data).chain_err(|| "json parsing error")?;
    v.is_object().as_result(v, "json does not include an object at toplevel".into())
}

fn is_json_file(path: String) -> std::result::Result<(), String>
{
    File::open(&path).map_err(|_| String::from("file not found"))?;
    if path.ends_with(".json") {
        std::result::Result::Ok(())
    } else {
        std::result::Result::Err("file must be json".into())
    }
}

// print a csv report showing properties by file to stdout
fn print_csv_report(input_files: &[&str], parsed_maps: &[&mut JsonMap]) -> Result<()> {
        let mut writer = csv::Writer::from_writer(std::io::stdout());

        // header
        let header = std::iter::once("property").chain(input_files.iter().cloned());
        writer.write_record(header).chain_err(|| "couldn't write record")?;

        // list of all properties
        let all_props: HashSet<&String> = HashSet::from_iter(parsed_maps.iter().flat_map(|obj| obj.keys()));
        for property in &all_props
        {
            let values = parsed_maps.iter()
                .map(|obj| serde_json::to_string(obj.get(*property).unwrap_or(&json!("--not present--"))).unwrap());

            let record = std::iter::once(property.to_string()).chain(values);
            writer.write_record(record).chain_err(|| "couldn't write record")?;
        }

        writer.flush()?;

        Ok(())
}

fn main() -> Result<()>
{
    let matches = App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(Arg::with_name("unique")
        .help("keep only properties which are unique in at least one input file")
        .short("u")
        .required_unless("same")
        .display_order(1)
        .takes_value(false))
    .arg(Arg::with_name("same")
        .help("keep only properties with the same value in all input files")
        .short("s")
        .conflicts_with("unique")
        .required_unless("unique")
        .display_order(1)
        .takes_value(false))
    .arg(Arg::with_name("input")
        .help("the input.json files to use")
        .required(true)
        .multiple(true)
        .min_values(2)
        .validator(is_json_file))
    .get_matches();

    // Here we can call .unwrap() because the argument is required.
    let unique = matches.is_present("unique");
    let input = matches.values_of("input").unwrap();
    let input_files: Vec<&str> = input.clone().collect();
    if input.clone().unique().count() < input.clone().count() { bail!("duplicate input file paths"); }

    let mut parsed = input.clone()
        .map(|path| parse_file(path))
        .collect::<Result<Vec<serde_json::Value>>>()?;

    {
        let mut parsed_maps = parsed.iter_mut()
            .map(|v| v.as_object_mut().ok_or_else(|| "json does not include an object at toplevel".into()))
            .collect::<Result<Vec<&mut JsonMap>>>()?;

        // check which properties are the same in all files
        let reference_properties: Vec<String> = parsed_maps[0].keys().cloned().collect();
        for property in &reference_properties
        {
            let contained_in_all_and_same =
            {
                let reference_value = &parsed_maps[0][property];
                parsed_maps.iter()
                    .skip(1)
                    .all(|obj| obj.contains_key(property) && &obj[property] == reference_value)
            };

            // if unique: remove properties that are contained_in_all_and_same
            // if !unique: remove properties that are !contained_in_all_and_same
            if !(contained_in_all_and_same ^ unique)
            {
                for obj in &mut parsed_maps
                {
                    obj.remove(property).chain_err(|| "internal error: non-existent property marked for purge")?;
                }
            }
        }

        if unique {
            return print_csv_report(&input_files, &parsed_maps);
        }
    }

    // not unique
    assert!(!unique);
    print!("{}", parsed[0].to_string());
    Ok(())
}
