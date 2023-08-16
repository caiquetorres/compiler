use std::collections::HashMap;

pub struct ParsedOptions {
    parsed_options: HashMap<String, String>,
}

impl ParsedOptions {
    pub fn new(parsed_options: HashMap<String, String>) -> Self {
        Self {
            parsed_options: parsed_options.clone(),
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.parsed_options.get(key).is_some()
    }

    pub fn get(&self, key: &str) -> Result<&str, &str> {
        self.parsed_options
            .get(key)
            .ok_or("The key does not exist")
            .map(|x| x.as_str())
    }
}
