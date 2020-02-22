use crate::parser::ParseData;
use crate::parser::ParsedData;
use crate::VariableType;
use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
pub trait SetStructure {
    fn set_searched_key(&mut self, value: String);
    fn set_composite_key(&mut self, value: Vec<String>);
    fn set_key_lists(&mut self, value: Vec<String>);
    fn set_parsed_data(&mut self, value: std::rc::Rc<ParsedData>);
}

impl SetStructure for StructuredData {
    fn set_searched_key(&mut self, value: String) {
        self.searched_key = value;
    }
    fn set_composite_key(&mut self, value: Vec<String>) {
        self.composite_key = value;
    }
    fn set_key_lists(&mut self, mut value: Vec<String>) {
        self.key_lists = vec!["config".to_string()];
        self.key_lists.append(&mut value);
    }
    fn set_parsed_data(&mut self, value: std::rc::Rc<ParsedData>) {
        self.parsed_data = value;
    }
}

#[derive(Deserialize, Debug, Default, Serialize)]
pub struct StructuredData {
    #[serde(skip_serializing, skip_deserializing)]
    searched_key: String,
    #[serde(skip_serializing, skip_deserializing)]
    composite_key: Vec<String>,
    #[serde(skip_serializing, skip_deserializing)]
    key_lists: Vec<String>,
    data: HashMap<String, HashMap<String, VariableType>>,
    #[serde(skip_serializing, skip_deserializing)]
    parsed_data: Rc<ParsedData>,
}
impl StructuredData {
    pub fn new() -> StructuredData {
        Default::default()
    }

    pub fn get_data<'a>(&'a self) -> &'a HashMap<String, HashMap<String, VariableType>> {
        &self.data
    }

    fn _build_index_key(&mut self, map: &HashMap<String, String>) -> String {
        let mut value_vec: Vec<String> = [].to_vec();
        for key in self.composite_key.iter() {
            if let Some(value) = map.get::<str>(&key.to_string()) {
                value_vec.push(value.to_string());
            }
        }

        /*for (key, value) in map {
            //if key exists in hashmap
            if self.composite_key.iter().any(|v| v == &key) {
                value_vec.push(value.clone());
                println!("key_used: {}, value_used:{}", key, value);
            }
        }*/
        value_vec.join(".").to_string()
    }

    pub fn calculate(&mut self) {
        let mut values_vec = Vec::new();
        for parsed_element in self.parsed_data.get_parsed_data().iter() {
            let mut curr_map: HashMap<String, String> = HashMap::new();
            let mut curr_line = String::from("");
            if let Some(s) = parsed_element.get("parsed_line") {
                if let VariableType::HashMap(map) = s {
                    curr_map = map.clone(); //not the best way, need to figure out how to get it out as ref
                                            //   values_vec.push(curr_map);
                };
            }
            if let Some(s) = parsed_element.get("line") {
                if let VariableType::String(string) = s {
                    curr_line = string.to_string(); //not the best way, need to figure out how to get it out as ref
                };
            }
            values_vec.push((curr_map, curr_line));
        }
        for (map, line) in values_vec.iter_mut() {
            self._process_change(&map, line);
        }
    }

    fn _process_change(&mut self, map: &HashMap<String, String>, line: &str) {
        let composite_index = self._build_index_key(&map);

        if map.contains_key(&self.searched_key.to_string()) {
            let element = self
                .data
                .entry(composite_index)
                .or_insert_with(HashMap::new);

            for (key, value) in map {
                if self.key_lists.contains(&key) {
                    let inner_element = element
                        .entry(key.to_string())
                        .or_insert_with(|| VariableType::List(vec![]));
                    if let VariableType::List(c) = inner_element {
                        c.push(value.to_string());
                    };
                } else {
                    element
                        .entry(key.to_string())
                        .or_insert_with(|| VariableType::String(value.to_string()));
                }
            }

            let inner_element = element
                .entry("config".to_string())
                .or_insert_with(|| VariableType::List(vec![]));

            if let VariableType::List(c) = inner_element {
                c.push(line.to_string());
            };
        }
    }
}
