mod config;
mod modules;
mod status;

use std::{
    thread,
    time::{Duration, Instant},
};

use crate::modules::ModuleData;

fn main() {
    let module_data = ModuleData::new(".config/simple_status/config.toml");

    let status_bar = status::Status::new();
    let mut time_point: Option<Instant> = None;

    loop {
        if time_point.is_none() || time_point.unwrap().elapsed().as_millis() >= 500 {
            status_bar.set_status(module_data.get_bar());
            time_point = Some(Instant::now());
        }

        thread::sleep(Duration::from_millis(1));
    }
}
