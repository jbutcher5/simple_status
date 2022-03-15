extern crate chrono;

use chrono::prelude::*;
use rayon::prelude::*;
use std::time::Instant;
use sysinfo::{ProcessorExt, System, SystemExt};

use crate::config::{Module, StatusConfig};

pub struct ModuleData {
    sys: System,
    bar: Option<Vec<Option<String>>>,
    time_point: Option<Instant>,
    modules: Vec<Module>,
    seperator: String,
}

impl ModuleData {
    pub fn new(config_path: &str) -> Self {
        let config_file = home::home_dir().unwrap().join(config_path);
        let config = config_file.get_config();

        Self {
            sys: System::new(),
            bar: None,
            time_point: None,
            modules: config.get_modules(),
            seperator: config.seperator,
        }
    }

    pub fn get_bar(&mut self) -> String {
        // Return the status bar as a string to be used by Status

        // Refresh all built-in modules that are used and require a refesh
        self.dynamic_refresh();

        let results: Vec<Option<String>> = if self.time_point.is_some() && self.bar.is_some() {
            let result: Vec<(Option<String>, Module)> = self
                .modules
                .par_iter()
                .zip(self.bar.as_ref().unwrap())
                .map(|x| -> (Option<String>, Module) {
                    let now = self.time_point.unwrap().elapsed().as_millis();
                    let delay: u128 = x.0.config.delay.unwrap().into();
                    let last_update: u128 = x.0.last_update;
                    let mut new_module = x.0.clone();
                    new_module.last_update = now;

                    if now >= delay + last_update && x.0.config.update {
                        (x.0.get(self), new_module)
                    } else {
                        (x.1.clone(), x.0.clone())
                    }
                })
                .collect();

            self.modules = result.iter().fold(vec![], |mut acc, x| {
                acc.push(x.1.clone());
                acc
            });
            result.iter().fold(vec![], |mut acc, x| {
                acc.push(x.0.clone());
                acc
            })
        } else {
            let result: Vec<Option<String>> = self
                .modules
                .par_iter()
                .map(|x| -> Option<String> { x.get(self) })
                .collect();

            self.time_point = Some(Instant::now());

            result
        };

        self.bar = Some(results.clone());

        let clean_results: Vec<String> = results
            .iter()
            .filter(|x| x.is_some())
            .cloned()
            .map(|x| x.unwrap())
            .collect();

        let result = clean_results.iter().fold(String::new(), |acc, x| {
            format!("{} {} {}", acc, self.seperator, x)
        });

        if result.len() <= self.seperator.len() {
            return String::new();
        }

        result[self.seperator.len() + 1..].trim().to_string()
    }

    pub fn dynamic_refresh(&mut self) {
        let built_in: Vec<String> = self
            .modules
            .iter()
            .filter(|x| x.config.built_in.is_some())
            .map(|x| x.config.built_in.as_ref().unwrap().clone())
            .collect();

        let has = |x: &Vec<String>, y: &[&str]| x.iter().any(|z| y.contains(&z.as_str()));

        if has(&built_in, &["cpu"]) {
            self.sys.refresh_cpu();
        }
        if has(&built_in, &["mem"]) {
            self.sys.refresh_memory();
        }
    }

    pub fn uptime_string(&self) -> String {
        let naive = NaiveDateTime::from_timestamp(self.sys.uptime().try_into().unwrap(), 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        datetime.format("%H:%M:%S").to_string()
    }

    pub fn date(&self) -> String {
        let local = Local::now();

        let day_num = local.format("%d").to_string();
        let suffix = match day_num.chars().last() {
            Some('1') => "st",
            Some('2') => "nd",
            Some('3') => "rd",
            Some(_) => "th",
            _ => "",
        };

        format!("{}{} {}", day_num, suffix, local.format("%b %y"))
    }

    pub fn time(&self) -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    pub fn memory_used(&self) -> String {
        let percentage = (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100f64;

        format!("{:.2}%", percentage)
    }

    pub fn load(&self) -> String {
        format!("{}", self.sys.load_average().one)
    }

    pub fn load_all(&self) -> String {
        format!(
            "{}, {}, {}",
            self.load(),
            self.sys.load_average().five,
            self.sys.load_average().fifteen,
        )
    }

    pub fn cpu(&self) -> String {
        let cores = self.sys.processors().iter().map(|x| x.cpu_usage());

        let total = cores.clone().fold(0_f32, |acc, x| acc + x);
        let avg = total / cores.len() as f32;

        format!("{:.2}%", avg)
    }
}
