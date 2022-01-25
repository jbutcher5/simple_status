mod config;
mod modules;
mod status;

use home;

use rayon::prelude::*;
use std::{
    thread,
    time::{Duration, Instant},
};

use crate::{config::StatusConfig, modules::ModuleData};

fn main() {
    let user_home = home::home_dir()
        .unwrap()
        .join(".config/simple_status/config.toml");
    let config = user_home.get_config();

    let mut module_data = ModuleData::new(config.clone());

    let status_bar = status::Status::new();
    let mut time_point: Option<Instant> = None;

    loop {
        if time_point.is_none() || time_point.unwrap().elapsed().as_millis() >= 500 {
            module_data.dynamic_refresh();

            let results: Vec<String> = config
                .modules
                .par_iter()
                .map(|x| -> String { module_data.translate(x.to_string()).unwrap_or_default() })
                .collect();

            let data: String = results.iter().fold(String::new(), |acc, x| {
                format!("{} {} {}", acc, &config.seperator, x)
            })[config.seperator.len() + 2..]
                .to_string();

            status_bar.set_status(data);
            time_point = Some(Instant::now());
        }

        thread::sleep(Duration::from_millis(1));
    }
}
