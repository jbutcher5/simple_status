mod modules;
mod status;

use std::{thread, time};
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    let active_modules = [
        modules::StatusModules::time,
        modules::StatusModules::uptime_string,
        modules::StatusModules::cpu,
        modules::StatusModules::memory_used,
    ];

    let prefixes = ["Time -", "Uptime -", "CPU -", "Mem -"];
    let seperator = "|";

    let mut x = status::Status::new(String::new());

    loop {
        sys.refresh_all();
        let data: String = active_modules
            .iter()
            .zip(prefixes)
            .fold(String::new(), |acc, x| {
                format!("{} {} {} {}", acc, seperator, x.1, x.0(&sys))
            });

        x.data = data;
        x.set_status();

        thread::sleep(time::Duration::from_millis(500));
    }
}
