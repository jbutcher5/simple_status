extern crate chrono;

use std::process::Command;

use chrono::prelude::*;
use sysinfo::{ProcessorExt, System, SystemExt};

pub struct Module {
    pub name: String,
    pub command: String,
}

impl Module {
    fn new(name: String, command: String) -> Self {
        Self { name, command }
    }

    fn stdout(&self) -> String {
        let seperate = self.command.split(" ").collect::<Vec<&str>>();

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

pub trait StatusModules {
    fn translate(
        &self,
        module: String,
        names: Vec<String>,
        commands: Vec<String>,
    ) -> Option<String>;
    fn dynamic_refresh(&mut self, modules: &Vec<String>);

    fn uptime_string(&self) -> String;
    fn time(&self) -> String;
    fn memory_used(&self) -> String;
    fn load(&self) -> String;
    fn load_all(&self) -> String;
    fn cpu(&self) -> String;
}

impl StatusModules for System {
    fn translate(
        &self,
        module: String,
        names: Vec<String>,
        commands: Vec<String>,
    ) -> Option<String> {
        let modules = names
            .iter()
            .zip(commands)
            .map(|x| -> Module {Module::new(x.0.to_owned(), x.1.to_string())})
            .filter(|x| x.name == module)
            .next();

        let result = match modules {
            Some(_) => modules.unwrap().stdout(),
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

        Some(result)
    }

    fn dynamic_refresh(&mut self, modules: &Vec<String>) {
        let has = |x: &Vec<String>, y: &[&str]| x.iter().any(|z| y.contains(&z.as_str()));

        if has(modules, &["cpu"]) {
            self.refresh_cpu();
        }
        if has(modules, &["mem"]) {
            self.refresh_memory();
        }
    }

    fn uptime_string(&self) -> String {
        let naive = NaiveDateTime::from_timestamp(self.uptime().try_into().unwrap(), 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        datetime.format("%H:%M:%S").to_string()
    }

    fn time(&self) -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    fn memory_used(&self) -> String {
        let percentage = (self.used_memory() as f64 / self.total_memory() as f64) * 100f64;

        format!("{:.2}%", percentage)
    }

    fn load(&self) -> String {
        format!("{}", self.load_average().one)
    }

    fn load_all(&self) -> String {
        format!(
            "{}, {}, {}",
            self.load(),
            self.load_average().five,
            self.load_average().fifteen,
        )
    }

    fn cpu(&self) -> String {
        let cores = self.processors().iter().map(|x| x.cpu_usage());

        let total = cores.clone().fold(0_f32, |acc, x| acc + x);
        let avg = total / cores.len() as f32;

        format!("{:.2}%", avg)
    }
}
