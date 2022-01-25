use serde_derive::Deserialize;
use toml;

use std::process::Command;
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

impl Module {
    fn stdout(&self) -> String {

        if self.command.is_none() {
            return String::new();
        }

        let seperate = self.command.unwrap().split(" ").collect::<Vec<&str>>();

        String::from_utf8(
            Command::new(seperate[0])
                .args(&seperate[1..])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .replace("\n", "")
        .trim()
        .to_string()
    }
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
