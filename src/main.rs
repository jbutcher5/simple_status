mod config;
mod modules;
mod status;

use home;

use rayon::prelude::*;
use std::{thread, time::{Duration, Instant}};
use sysinfo::{System, SystemExt};

use crate::{config::StatusConfig, modules::StatusModules};

fn main() {
    let mut sys = System::new();

    let user_home = home::home_dir()
        .unwrap()
        .join(".config/simple_status/config.toml");
    let config = user_home.get_config();

    let status_bar = status::Status::new();
    let mut time_point: Option<Instant> = None;

    loop {
        if time_point.is_none() || time_point.unwrap().elapsed().as_millis() >= 500 {
            sys.dynamic_refresh(&config.modules);

            let results: Vec<String> = config
                .modules
                .par_iter()
                .map(|x| -> String {
                    sys.translate(
                        x.to_string(),
                        config.module_names.clone(),
                        config.module_commands.clone(),
                    )
                    .unwrap_or_default()
                })
                .collect();

            let data: String = config
                .prefixes
                .iter()
                .zip(&results)
                .fold(String::new(), |acc, x| {
                    format!("{} {} {} {}", acc, config.seperator, x.0, x.1)
                })[config.seperator.len() + 2..]
                .to_string();

            status_bar.set_status(data);
            time_point = Some(Instant::now());
        }

        thread::sleep(Duration::from_millis(1));
    }
}
