use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct GmUriParser {
    log_regex: Regex,
    script_mappings: HashMap<String, String>,
    cache_functions_to_files: HashMap<String, String>,
}

impl GmUriParser {
    pub fn new<P: AsRef<Path>>(scripts_directory: P) -> Self {
        let log_regex = Regex::new(r"gml_(?:Object|Script).+?:\d+").unwrap();

        let mut script_mappings = HashMap::new();

        let func_finder = Regex::new(r#"(?m)^function\s+(\w+)"#).unwrap();

        for file in WalkDir::new(scripts_directory)
            .into_iter()
            .filter_map(|v| v.ok())
        {
            if file.path().extension().and_then(|v| v.to_str()) == Some("gml") {
                if let Some(stem) = file.path().file_stem() {
                    if let Ok(file_data) = std::fs::read_to_string(file.path()) {
                        let file_name = stem.to_str().unwrap();
                        script_mappings.insert(file_name.to_string(), file_name.to_string());
                        func_finder.captures_iter(&file_data).for_each(|v| {
                            let (_, [func_name]) = v.extract();
                            script_mappings
                                .insert(func_name.to_string(), stem.to_str().unwrap().to_string());
                        })
                    }
                }
            }
        }

        Self {
            log_regex,
            script_mappings,
            cache_functions_to_files: HashMap::new(),
        }
    }

    pub fn parse(&mut self, input: &mut String) {
        if let Some(log) = self.log_regex.find(input).map(|v| v.as_str()) {
            let x = gml_log_parser::parse(log, &self.script_mappings);
            if let Ok(result) = x {
                *input = input.replace(log, &result);
            }
        }
    }
}
