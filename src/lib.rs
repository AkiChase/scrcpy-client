use std::process::exit;

pub mod util;
pub mod client;

pub fn check_resource() {
    if let Err(val) = util::ResHelper::res_init() {
        eprintln!("{}", val);
        exit(1);
    }
}
