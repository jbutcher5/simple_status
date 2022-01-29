extern crate chrono;

use home;

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
        self.dynamic_refresh();

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

        clean_results.iter().fold(String::new(), |acc, x| {
            format!("{} {} {}", acc, &self.config.seperator, x)
        })[self.config.seperator.len() + 2..]
            .to_string()
    }

    fn translate(&self, module: String) -> Option<String> {
        let module_data = &self.config.module[&module];

        let result: String = match module_data.command {
            Some(_) => module_data.stdout(),
            None => match module.as_str() {
                "cpu" => self.cpu(),
                "mem" => self.memory_used(),
                "uptime" => self.uptime_string(),
                "time" => self.time(),
                "load" => self.load(),
                "load_all" => self.load_all(),
                _ => return None,
            },
        };

        if result == ""{
            return None
        }

        Some(format!("{} {}", module_data.prefix, result))
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
