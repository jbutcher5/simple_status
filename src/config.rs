use serde_derive::Deserialize;
use derivative::Derivative;
use std::{fs, path::PathBuf, process::Command};

use crate::modules::ModuleData;

fn _true() -> bool {
    true
}

#[derive(Derivative, Deserialize, Clone, Debug)]
pub struct Config {
    pub seperator: String,
    pub module: Vec<ConfigModule>,
    pub default_delay: Option<u64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConfigModule {
    pub command: Option<String>,
    pub built_in: Option<String>,
    pub prefix: Option<String>,

    #[serde(default)]
    pub delay: u64,

    #[serde(default = "_true")]
    pub update: bool,
}

#[derive(Clone)]
pub struct Module {
    pub config: ConfigModule,
    pub last_update: u128,
}


impl Config {
    pub fn get_modules(&self) -> Vec<Module> {
        self.module
            .iter()
            .map(|x| Module::new(x.clone()))
            .collect()
    }
}

impl Module {
    pub fn new(config: ConfigModule) -> Self {
        Self {
            config,
            last_update: 0,
        }
    }

    pub fn get(&self, module_data: &ModuleData) -> Option<String> {
        let result: Option<String> = match self.config.command {
            Some(_) => self.stdout(),
            None => Some(match self.config.built_in.as_ref().unwrap().as_str() {
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

        match self.config.prefix {
            Some(_) => Some(format!("{} {}", self.config.prefix.as_ref()?, result?)),
            None => Some(result?),
        }
    }

    pub fn stdout(&self) -> Option<String> {
        let seperate = match self.config.command {
            Some(_) => self
                .config
                .command
                .as_ref()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>(),
            None => return None,
        };

        let command = Command::new(seperate[0]).args(&seperate[1..]).output();

        let stdout = match command {
            Ok(_) => command.unwrap().stdout,
            Err(_) => return None,
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
