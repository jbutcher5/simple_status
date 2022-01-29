use serde_derive::Deserialize;

use std::{collections::HashMap, fs, path::PathBuf, process::Command};

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub seperator: String,
    pub module: HashMap<String, Module>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Module {
    pub command: Option<String>,
    pub prefix: Option<String>,
}

impl Module {
    pub fn stdout(&self) -> Option<String> {
        let seperate = match self.command {
            Some(_) => self
                .command
                .as_ref()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>(),
            _ => return None,
        };

        let command = Command::new(seperate[0]).args(&seperate[1..]).output();

        let stdout = match command {
            Ok(_) => command.unwrap().stdout,
            _ => return None,
        };

        Some(
            String::from_utf8(stdout)
                .unwrap()
                .replace('\n', "")
                .trim()
                .to_string(),
        )
    }
}

pub trait StatusConfig {
    fn get_config(&self) -> Config;
}

impl StatusConfig for PathBuf {
    fn get_config(&self) -> Config {
        let content = fs::read_to_string(self.as_path().to_str().unwrap())
            .expect("Something went wrong reading the config file.");

        toml::from_str(content.as_str()).unwrap()
    }
}
