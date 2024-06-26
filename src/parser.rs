use super::file_loader::*;
use crate::VariableType;
use fancy_regex::Regex;
//use std::process::*;
extern crate rayon;
//use serde::{Deserialize, Serialize};
use crate::parser::rayon::iter::IntoParallelRefIterator;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::ParallelIterator;
use std::collections::HashMap;
use std::sync::Arc;

pub trait ParseData {
    fn new() -> ParsedData;
    fn parse(&mut self);
    fn _parse_line(
        compiled_regex_arc: &Arc<Vec<Result<fancy_regex::Regex, fancy_regex::Error>>>,
        line: &str,
    ) -> HashMap<String, VariableType>;
    fn set_regex_hashmap(&mut self, map: HashMap<String, String>);
    fn set_source_config(&mut self, config: &clap::ArgMatches<'static>);
    fn get_parsed_data(&self) -> &Vec<HashMap<String, VariableType>>;
}

#[derive(Debug, Default)]
pub struct ParsedData {
    source_config: Vec<String>,
    data: Vec<HashMap<String, VariableType>>,
    regex_hashmap: HashMap<String, String>,
}

impl ParseData for ParsedData {
    fn new() -> ParsedData {
        Default::default()
    }
    fn get_parsed_data(&self) -> &Vec<HashMap<String, VariableType>> {
        &self.data
    }
    fn _parse_line(
        compiled_regex_arc: &Arc<Vec<Result<fancy_regex::Regex, fancy_regex::Error>>>,
        line: &str,
    ) -> HashMap<String, VariableType> {
        let mut result: HashMap<String, String> = HashMap::new();
        let mut curr_map: HashMap<String, VariableType> = HashMap::new();
        for regex in compiled_regex_arc.iter() {
            match regex {
                Ok(re) => {
                    for (_index, capture_name) in re.capture_names().enumerate() {
                        if let Some(name) = capture_name {
                            let captures = re.captures(&line).unwrap();
                            if let Some(found_capture) = captures {
                                result.insert(
                                    name.to_string(),
                                    String::from(found_capture.name(name).unwrap().as_str()),
                                );
                            }
                        }
                    }
                }
                Err(re) => {
                    eprintln!("Unable to parse {:#?}", re);
                }
            }
        }
        curr_map
            .entry("line".to_string())
            .or_insert_with(|| VariableType::String(line.to_string()));
        curr_map
            .entry("parsed_line".to_string())
            .or_insert_with(|| VariableType::HashMap(result));
        curr_map
    }

    fn set_source_config(&mut self, config: &clap::ArgMatches<'static>) {
        let mut loader = FileLoader::new();
        if let Some(c) = config.value_of("config") {
            loader.set_path(&c);
        }
        fn get_string_vec(input_string: &str) -> Vec<String> {
            let res: Vec<String> = input_string.split('\n').map(|s| s.to_string()).collect();
            res
        }
        self.source_config = get_string_vec(loader.get_content());
    }
    fn parse(&mut self) {
        let pb = ProgressBar::new(self.source_config.len() as u64);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} [Parsing configuration {elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        ));

        let regex = &self.regex_hashmap;
        let compiled_regex: Vec<Result<fancy_regex::Regex, fancy_regex::Error>> = regex
            .iter()
            .map(|(_key, value)| Regex::new(value))
            .collect();
        let compiled_regex_arc: Arc<Vec<Result<fancy_regex::Regex, fancy_regex::Error>>> =
            Arc::new(compiled_regex);
        let results: Vec<HashMap<String, VariableType>> = self
            .source_config
            .clone()
            .par_iter()
            .progress_with(pb)
            .map(|line| ParsedData::_parse_line(&compiled_regex_arc, &line))
            .collect();

        self.data = results;
    }

    fn set_regex_hashmap(&mut self, map: HashMap<String, String>) {
        self.regex_hashmap = map;
    }
}
