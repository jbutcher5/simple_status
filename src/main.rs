mod modules;
mod status;

use std::{thread, time};
use sysinfo::{System, SystemExt};

use crate::modules::StatusModules;

fn main() {
    let mut sys = System::new();

    let enabled_modules = ["load", "time", "uptime", "cpu", "mem"];
    let prefixes = ["華 ", " ", " ", " ", " "];
    let seperator = " ";

    let mut x = status::Status::new(String::new());

    loop {
        sys.dynamic_refresh(&enabled_modules);
        let data: String = enabled_modules
            .iter()
            .zip(prefixes)
            .fold(String::new(), |acc, x| {
                format!(
                    "{} {} {} {}",
                    acc,
                    seperator,
                    x.1,
                    sys.translate(x.0).unwrap_or(String::new())
                )
            })[seperator.len()+2..].to_string();

        x.data = data;
        x.set_status();

        thread::sleep(time::Duration::from_millis(500));
    }
}
