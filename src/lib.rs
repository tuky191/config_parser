extern crate clap;
extern crate onig;

use crate::parser::ParseData;
use crate::structer::SetStructure;
use crate::structer::StructuredData;
use clap::{App, Arg};
use std::collections::HashMap;
mod file_loader;
mod output;
mod parser;
mod structer;
use file_loader::FileLoader;
use output::OuputFormatter;
use output::OutputFormat;
//use indicatif::{ProgressBar, ProgressStyle};
//use log::{info, warn};
use log::info;
use serde::{Deserialize, Serialize};
//use serde_json::Result;
use serde_json::{Map, Result, Value};
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
extern crate log;
/*
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map:HashMap<String, String> = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
*/
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VariableType {
    List(Vec<String>),
    String(String),
    HashMap(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonConfig {
    name: String,
    searched_key: String,
    composite_key: Vec<String>,
    key_lists: Vec<String>,
}

fn load_regex_from_file(config: &clap::ArgMatches<'static>) -> Result<HashMap<String, String>> {
    let mut loader = FileLoader::new();
    if let Some(c) = config.value_of("regex") {
        loader.set_path(&c);
    }
    let value_vec: Vec<Value> = serde_json::from_str(loader.get_content())?;
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for item in value_vec.iter() {
        let map: Map<String, Value> = item.as_object().unwrap().clone();
        for (key, value) in map {
            hashmap.insert(key.to_string(), value.as_str().unwrap().to_string());
        }
    }
    Ok(hashmap)
}

fn load_structure_from_file(config: &clap::ArgMatches<'static>) -> Result<Vec<JsonConfig>> {
    let mut loader = FileLoader::new();
    if let Some(c) = config.value_of("structure") {
        loader.set_path(&c);
    }
    let content = loader.get_content();
    let v: Vec<JsonConfig> = serde_json::from_str(content)?;
    Ok(v)
}

pub fn run() -> Result<()> {
    info!("Loading up structure template and regex mappings");
    let mut parsed_data_obj = parser::ParsedData::new();
    let config = get_input_parameters();
    let config_hashmap = load_structure_from_file(&config)?;
    let regex_hashmap = load_regex_from_file(&config)?; // need to remove unwrap and handle fault state
                                                        // println!("{:#?}", regex_hashmap);
                                                        //process::exit(1);
                                                        //pb.finish_with_message("Loading up structure template and regex mappings... Done!");
                                                        //
    info!("Parsing configuration file");
    parsed_data_obj.set_source_config(&config);
    parsed_data_obj.set_regex_hashmap(regex_hashmap);
    parsed_data_obj.parse();
    info!("Applying Template on parsed data");
    //println!("{:#?}", parsed_data_obj);
    let rc_parsed_data_obj = Rc::new(parsed_data_obj);
    let mut objects_hashmap: HashMap<&String, StructuredData> = HashMap::new();
    for element in config_hashmap.iter() {
        objects_hashmap.insert(&element.name, StructuredData::new());
        if let Some(reff) = objects_hashmap.get_mut(&element.name) {
            reff.set_parsed_data(rc_parsed_data_obj.clone());
            reff.set_searched_key(element.searched_key.clone());
            reff.set_composite_key(element.composite_key.clone());
            reff.set_key_lists(element.key_lists.clone());
            reff.calculate();
        }
    }
    let mut output_format = OutputFormat::new();
    output_format.set_map(&config_hashmap, &objects_hashmap);
    output_format.set_format(
        config
            .value_of("format")
            .expect("Unable to set appropriate format"),
    );
    if let Some(c) = config.value_of("output") {
        if c == "FILE" {
            let mut filename = String::from("parsed_config.");
            let format = config.value_of("format").unwrap().to_lowercase();
            filename.push_str(&format);
            let handle = File::create(&filename);
            if let Ok(mut f) = handle {
                f.write_all(output_format.formatted_output.as_bytes())
                    .expect("Unable to write data");
            }
        }
        if c == "STDOUT" {
            println!("{}", output_format.formatted_output);
        }
    }
    Ok(())
}

fn get_input_parameters() -> clap::ArgMatches<'static> {
    App::new("ConfigParser")
        .version("0.1")
        .author("Michal T")
        .about("Tool to parse router set config to structured data")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .required(true)
                .value_name("FILE")
                .help("Path to the set configuration file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE|STDOUT")
                .possible_values(&["FILE", "STDOUT"])
                .default_value("STDOUT")
                .help("Send output to FILE or STDOUT"),
        )
        .arg(
            Arg::with_name("structure")
                .short("s")
                .long("structure")
                .required(true)
                .value_name("JSON")
                .help("Path to the JSON formatted file containing required output structure")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("regex")
                .short("r")
                .long("regex")
                .required(true)
                .value_name("JSON")
                .help("Path to the JSON formatted key / value regex file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("JSON|YAML")
                .help("Set output format, defaults to JSON")
                .possible_values(&["JSON", "YAML"])
                .default_value("JSON")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .multiple(true)
                .help("Turn debugging information on"),
        )
        .get_matches()
}
