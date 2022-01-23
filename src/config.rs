use serde_derive::Deserialize;
use serde_yaml;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub prefixes: Vec<String>,
    pub seperator: String,
    pub module_names: Vec<String>,
    pub module_commands: Vec<String>,
}

pub trait StatusConfig {
    fn get_config(&self) -> Config;
}

impl StatusConfig for PathBuf {
    fn get_config(&self) -> Config {
        let content = fs::read_to_string(self.as_path().to_str().unwrap())
            .expect("Something went wrong reading the config file.");

        let result: Config = serde_yaml::from_str(content.as_str()).unwrap();
        result
    }
}
