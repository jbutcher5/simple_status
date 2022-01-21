use std::path::PathBuf;
use toml;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub modules: Vec<String>,
    pub prefixes: Vec<String>,
    pub seperator: String
}

use std::fs;

pub trait StatusConfig {
    fn get_config(&self) -> Config;
}

impl StatusConfig for PathBuf {
    fn get_config(&self) -> Config {
        let content = fs::read_to_string(self.as_path().to_str().unwrap())
            .expect("Something went wrong reading the config file.");

        let result: Config = toml::from_str(content.as_str()).unwrap();
        result
    }
}
