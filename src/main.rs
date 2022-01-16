mod status;
mod modules;

use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let x = status::Status::new(modules::uptime("Uptime:".to_string(), sys));
    x.set_status();
}
