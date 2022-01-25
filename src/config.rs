use serde_derive::Deserialize;
use toml;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub seperator: String,
    pub module: HashMap<String, Module>
}

#[derive(Deserialize, Debug)]
pub struct Module {
    pub command: Option<String>,
    pub prefix: String
}

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
