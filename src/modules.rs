extern crate chrono;

use chrono::prelude::*;
use sysinfo::{System, SystemExt};

pub fn uptime(prefix: String, sys: &System) -> String {
    let naive = NaiveDateTime::from_timestamp(sys.uptime().try_into().unwrap(), 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    format!("{} {}", prefix, datetime.format("%H:%M:%S"))
}

pub fn system_name(prefix: String, sys: &System) -> String {
    format!("{} {}", prefix, sys.name().unwrap())
}

pub fn kernel_version(prefix: String, sys: &System) -> String {
    format!("{} {}", prefix, sys.os_version().unwrap())
}

pub fn os_version(prefix: String, sys: &System) -> String {
    format!("{} {}", prefix, sys.os_version().unwrap())
}

pub fn host_name(prefix: String, sys: &System) -> String {
    format!("{} {}", prefix, sys.host_name().unwrap())
}

pub fn memory_used(prefix: String, sys: &System) -> String {
    let percentage = sys.used_memory() as f64/sys.total_memory() as f64;

    format!("{} {:.4}%", prefix, percentage)
}

pub fn load(prefix: String, sys: &System) -> String {
    format!("{} {}%", prefix, sys.load_average().one)
}