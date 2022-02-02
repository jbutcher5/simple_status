use serde_derive::Deserialize;

use std::{collections::HashMap, fs, path::PathBuf, process::Command};

use crate::modules::ModuleData;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub modules: Vec<String>,
    pub seperator: String,
    pub module: HashMap<String, Module>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Module {
    pub command: Option<String>,
    pub built_in: Option<String>,
    pub prefix: Option<String>,
}

impl Module {
    pub fn get(&self, module_data: &ModuleData) -> Option<String> {

        let result: Option<String> = match self.command {
            Some(_) => self.stdout(),
            None => Some(match self.built_in.as_ref().unwrap().as_str() {
                "cpu" => module_data.cpu(),
                "mem" => module_data.memory_used(),
                "uptime" => module_data.uptime_string(),
                "date" => module_data.date(),
                "time" => module_data.time(),
                "load" => module_data.load(),
                "load_all" => module_data.load_all(),
                _ => return None,
            }),
        };

        if result.is_none() || result.as_ref()?.is_empty() {
            return None;
        }

        match self.prefix {
            Some(_) => Some(format!("{} {}", .prefix.as_ref()?, result?)),
            _ => Some(result?),
        }
    }

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
