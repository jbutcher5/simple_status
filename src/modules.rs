extern crate chrono;

use sysinfo::{System, SystemExt};
use chrono::prelude::*;

pub fn uptime(prefix: String, sys: System) -> String {
    let naive = NaiveDateTime::from_timestamp(sys.uptime().try_into().unwrap(), 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    format!("{} {}", prefix, datetime.format("%H:%M:%S"))
}
