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

    let status_bar = status::Status::new();
    let mut time_point: Option<Instant> = None;

    loop {
        if time_point.is_none() || time_point.unwrap().elapsed().as_millis() >= 500 {
            sys.dynamic_refresh(&config.modules);
            let data: String =
                config.modules
                    .iter()
                    .zip(&config.prefixes)
                    .fold(String::new(), |acc, x| {
                        format!(
                            "{} {} {} {}",
                            acc,
                            config.seperator,
                            x.1,
                            sys.translate(
                                x.0.to_owned(),
                                config.module_names.clone(),
                                config.module_commands.clone()
                            )
                            .unwrap()
                        )
                    })[config.seperator.len() + 2..]
                    .to_string();

            status_bar.set_status(data);
            time_point = Some(Instant::now());
        }

        thread::sleep(Duration::from_millis(1));
    }
}
