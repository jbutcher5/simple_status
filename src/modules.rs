extern crate chrono;

use chrono::prelude::*;
use rayon::prelude::*;
use sysinfo::{ProcessorExt, System, SystemExt};

use crate::config::{Config, StatusConfig};

pub struct ModuleData {
    config: Config,
    sys: System,
}

impl ModuleData {
    pub fn new(config_path: &str) -> Self {
        let config_file = home::home_dir().unwrap().join(config_path);
        let config = config_file.get_config();

        Self {
            config,
            sys: System::new(),
        }
    }

    pub fn get_bar(&mut self) -> String {
        // Return the status bar as a string to be used by Status

        // Refresh all built-in modules that are used and require a refesh
        self.dynamic_refresh();

        // Thread the map statement on each item in the iterable
        let results: Vec<Option<String>> = self
            .config
            .modules
            .par_iter()
            .map(|x| -> Option<String> { self.translate(x.to_string()) })
            .collect();

        let clean_results: Vec<String> = results
            .iter()
            .filter(|x| !x.is_none())
            .cloned()
            .map(|x| x.unwrap())
            .collect();

        let result = clean_results
            .iter()
            .fold(String::new(), |acc, x| {
                format!("{} {} {}", acc, &self.config.seperator, x)
            })
            .to_string();

        if result.len() <= self.config.seperator.len() {
            return String::new();
        }

        result[self.config.seperator.len() + 1..].trim().to_string()
    }

    fn translate(&self, module: String) -> Option<String> {
        // Take a string from modules in the config file and find
        // the function that is referring to, be it a command or
        // command to be run.
        // TODO: Check if module is denfined self.config.module keys.

        let module_data = &self.config.module[&module];

        let potential_built_in: &str = module_data.built_in.as_ref().unwrap_or(&module);

        let result: Option<String> = match module_data.command {
            Some(_) => module_data.stdout(),
            None => Some(match potential_built_in {
                "cpu" => self.cpu(),
                "mem" => self.memory_used(),
                "uptime" => self.uptime_string(),
                "date" => self.date(),
                "time" => self.time(),
                "load" => self.load(),
                "load_all" => self.load_all(),
                _ => return None,
            }),
        };

        if result.is_none() || result.as_ref()?.is_empty() {
            return None;
        }

        match module_data.prefix {
            Some(_) => Some(format!("{} {}", module_data.prefix.as_ref()?, result?)),
            _ => Some(result?),
        }
    }

    fn dynamic_refresh(&mut self) {
        let has = |x: &Vec<String>, y: &[&str]| x.iter().any(|z| y.contains(&z.as_str()));

        if has(&self.config.modules, &["cpu"]) {
            self.sys.refresh_cpu();
        }
        if has(&self.config.modules, &["mem"]) {
            self.sys.refresh_memory();
        }
    }

    fn uptime_string(&self) -> String {
        let naive = NaiveDateTime::from_timestamp(self.sys.uptime().try_into().unwrap(), 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        datetime.format("%H:%M:%S").to_string()
    }

    fn date(&self) -> String {
        let local = Local::now();

        let day_num = local.format("%d").to_string();
        let suffix = match day_num.as_str() {
            "1" => "st",
            "2" => "nd",
            "3" => "rd",
            _ => "th",
        };

        format!("{}{} {}", day_num, suffix, local.format("%b %y"))
    }

    fn time(&self) -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    fn memory_used(&self) -> String {
        let percentage = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100f64;

        format!("{:.2}%", percentage)
    }

    fn load(&self) -> String {
        format!("{}", self.sys.load_average().one)
    }

    fn load_all(&self) -> String {
        format!(
            "{}, {}, {}",
            self.load(),
            self.sys.load_average().five,
            self.sys.load_average().fifteen,
        )
    }

    fn cpu(&self) -> String {
        let cores = self.sys.processors().iter().map(|x| x.cpu_usage());

        let total = cores.clone().fold(0_f32, |acc, x| acc + x);
        let avg = total / cores.len() as f32;

        format!("{:.2}%", avg)
    }
}
