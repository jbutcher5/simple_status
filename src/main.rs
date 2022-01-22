mod modules;
mod config;
mod status;

use std::{thread, time};
use sysinfo::{System, SystemExt};
use home;

use crate::{config::StatusConfig, modules::StatusModules};

fn main() {
    let mut sys = System::new();

    let user_home = home::home_dir().unwrap().join(".config/simple_status/config.yaml");
    let config = user_home.get_config();

    let enabled_modules = config.modules;
    let prefixes = config.prefixes;
    let seperator = config.seperator;

    let status_bar = status::Status::new();
    loop {
        sys.dynamic_refresh(&enabled_modules);
        let data: String = enabled_modules
            .iter()
            .zip(&prefixes)
            .fold(String::new(), |acc, x| {
                format!(
                    "{} {} {} {}",
                    acc,
                    seperator,
                    x.1,
                    sys.translate(x.0.to_owned()).unwrap_or(String::new())
                )
            })[seperator.len()+2..].to_string();

        status_bar.set_status(data);

        thread::sleep(time::Duration::from_millis(500));
    }
}
