mod status;

fn main() {
    let x = status::Status::new("test");
    x.set_status();
}
