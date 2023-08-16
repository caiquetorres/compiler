use std::collections::HashMap;

use super::parsed_options::ParsedOptions;

fn is_option(text: &str) -> bool {
    return text.starts_with("-") || text.starts_with("--");
}

pub struct CommandLineParser {
    short_options: HashMap<String, String>,
}

impl CommandLineParser {
    pub fn new() -> Self {
        CommandLineParser {
            short_options: HashMap::new(),
        }
    }

    pub fn add_option(&mut self, short_option: &str, option: &str) {
        self.short_options
            .insert(String::from(short_option), String::from(option));
    }

    pub fn parse(&self, args: &Vec<String>) -> ParsedOptions {
        let mut parsed_options: HashMap<String, String> = HashMap::new();

        for (i, mut key) in args.iter().enumerate() {
            key = self.short_options.get(key).unwrap_or(key);

            if i + i > args.len() - 1 {
                parsed_options.insert(String::from(key), String::from(""));
                continue;
            }

            let raw_str = args[i + 1].as_str();
            let value = String::from(if !is_option(raw_str) { raw_str } else { "" });

            parsed_options.insert(String::from(key), value);
        }

        ParsedOptions::new(parsed_options)
    }
}
