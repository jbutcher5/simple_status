extern crate chrono;

use chrono::prelude::*;
use sysinfo::{System, SystemExt};

pub trait StatusModules {
    fn uptime_string(&self) -> String;
    fn time(&self) -> String;
    fn memory_used(&self) -> String;
    fn load(&self) -> String;
    fn load_all(&self) -> String;
}

impl StatusModules for System {
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

        format!("{:.1}%", percentage)
    }

    fn load(&self) -> String {
        format!("{}", self.load_average().one)
    }

    fn load_all(&self) -> String {
        format!(
            "{}, {}, {}",
            self.load_average().one,
            self.load_average().five,
            self.load_average().fifteen,
        )
    }
}
