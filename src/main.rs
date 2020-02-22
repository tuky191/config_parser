use std::process;
fn main() {
    env_logger::init();

    if let Err(e) = config_parser::run() {
        eprintln!("Application error! {}", e);
        process::exit(1);
    }
}
