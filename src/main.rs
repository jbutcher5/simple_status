mod modules;
mod status;

use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let active_modules = [modules::uptime, modules::system_name];
    let prefixes = ["Uptime:", "OS:"];

    let data: String = active_modules
        .iter()
        .zip(prefixes)
        .fold(String::new(), |acc, x| {
            format!("{} {}", acc, x.0(x.1.to_string(), &sys))
        });

    let x = status::Status::new(data);
    x.set_status();
}
