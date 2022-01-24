mod config;
mod modules;
mod status;

use home;

use std::{thread, time::{Instant, Duration}};

use sysinfo::{System, SystemExt};

use crate::{config::StatusConfig, modules::StatusModules};

fn main() {
    let mut sys = System::new();

    let user_home = home::home_dir()
        .unwrap()
        .join(".config/simple_status/config.yaml");
    let config = user_home.get_config();

    let enabled_modules = config.modules;
    let prefixes = config.prefixes;
    let seperator = config.seperator;

    let status_bar = status::Status::new();

    let mut time_point: Option<Instant> = None;

    loop {
        if time_point.is_none() || time_point.unwrap().elapsed().as_millis() >= 500 {
            let module_names: Vec<String> = config.module_names.clone();
            let module_commands: Vec<String> = config.module_commands.clone();

            sys.dynamic_refresh(&enabled_modules);
            let data: String =
                enabled_modules
                    .iter()
                    .zip(&prefixes)
                    .fold(String::new(), |acc, x| {
                        format!(
                            "{} {} {} {}",
                            acc,
                            seperator,
                            x.1,
                            sys.translate(
                                x.0.to_owned(),
                                module_names.clone(),
                                module_commands.clone()
                            )
                            .unwrap()
                        )
                    })[seperator.len() + 2..]
                    .to_string();

            status_bar.set_status(data);
            time_point = Some(Instant::now());
        }

        thread::sleep(Duration::from_millis(1));
    }
}
