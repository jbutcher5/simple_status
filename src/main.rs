mod config;
mod modules;
mod status;

use std::{
    thread,
    time::{Duration, Instant},
};

use clap::{App, Arg, ArgMatches};

use crate::modules::ModuleData;

fn main() {
    // Get arguments
    let args = args();
    let config_path = args
        .value_of("CONFIG")
        .unwrap_or(".config/simple_status/config.toml");

    // Parse config into modules
    let mut module_data = ModuleData::new(config_path);

    // Create a connection to the display
    let status_bar = status::Status::new();

    // Create an initial time point
    let mut time_point: Option<Instant> = None;

    loop {
        if time_point.is_none() || time_point.unwrap().elapsed().as_millis() >= 500 {
            status_bar.set_status(module_data.get_bar());
            time_point = Some(Instant::now());
        }

        // Sleep the main thread for 1 millisecond to reduce cpu usage significantly
        thread::sleep(Duration::from_millis(1));
    }
}

fn args() -> ArgMatches {
    // Build Argument App with clap
    return App::new("simple_status")
        .version(env!("CARGO_PKG_VERSION"))
        .author("James Butcher <jamesbutcher@duck.com>")
        .about("Easy, Simple, Clean. A dwm modular config bar. Inspired by dwmblocks.")
        .arg(
            Arg::new("CONFIG")
                .short('c')
                .long("config")
                .takes_value(true)
                .help("Path to the configuration file"),
        )
        .get_matches();
}
