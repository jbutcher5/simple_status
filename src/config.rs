use toml::Value;
use std::fs;

pub trait StatusConfig {
    fn get_config(&self) -> Value;
}

impl StatusConfig for &str {
    fn get_config(&self) -> Value {
        let content = fs::read_to_string(self)
            .expect("Something went wrong reading the config file.");

        content.parse::<Value>().unwrap()
    }
}
