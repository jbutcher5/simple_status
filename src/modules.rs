extern crate chrono;

use chrono::prelude::*;
use sysinfo::{ProcessorExt, System, SystemExt};

pub trait StatusModules {
    fn translate(&self, module: &str) -> Option<String>;

    fn uptime_string(&self) -> String;
    fn time(&self) -> String;
    fn memory_used(&self) -> String;
    fn load(&self) -> String;
    fn load_all(&self) -> String;
    fn cpu(&self) -> String;
}

impl StatusModules for System {
    fn translate(&self, module: &str) -> Option<String> {
        let result = match module {
            "cpu" => self.cpu(),
            "mem" => self.memory_used(),
            "uptime" => self.uptime_string(),
            "time" => self.time(),
            "load" => self.load(),
            "load_all" => self.load_all(),
            _ => return None,
        };

        Some(result)
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
