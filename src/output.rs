use super::*;
#[derive(Debug)]
enum OutputFormats {
    Yaml,
    Json,
    None,
}

impl Default for OutputFormats {
    fn default() -> Self {
        OutputFormats::None
    }
}

#[derive(Debug, Default)]
pub struct OutputFormat {
    output_format: OutputFormats,
    output_map: HashMap<String, HashMap<String, HashMap<String, VariableType>>>,
    pub formatted_output: String,
}

pub trait OuputFormatter {
    fn new() -> OutputFormat;
    fn set_map(
        &mut self,
        config_hashmap: &[JsonConfig],
        objects_hashmap: &HashMap<&String, StructuredData>,
    );
    fn set_format(&mut self, format: &str);
}

impl OuputFormatter for OutputFormat {
    fn new() -> OutputFormat {
        Default::default()
    }

    fn set_format(&mut self, format: &str) {
        match format {
            "YAML" => {
                self.output_format = OutputFormats::Yaml;
                if let Ok(k) = ::serde_yaml::to_string(&self.output_map) {
                    self.formatted_output = k;
                }
            }
            "JSON" => {
                self.output_format = OutputFormats::Json;
                if let Ok(k) = ::serde_json::to_string(&self.output_map) {
                    self.formatted_output = k;
                }
            }
            &_ => {
                self.output_format = OutputFormats::None;
                eprint!("{:#?} is NOT SUPPORTED", format);
            }
        };
    }

    fn set_map<'a>(
        &'a mut self,
        config_hashmap: &'a [JsonConfig],
        objects_hashmap: &'a HashMap<&'a String, StructuredData>,
    ) {
        for element in config_hashmap.iter() {
            if let Some(reff) = objects_hashmap.get(&element.name) {
                self.output_map
                    .entry(element.name.to_string())
                    .or_insert_with(|| reff.get_data().clone());
            }
        }
    }
}
