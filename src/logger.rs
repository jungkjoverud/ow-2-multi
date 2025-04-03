use log::{info, error};

pub fn init_logger() {
    simple_logger::init().unwrap();
}

pub fn log_event(event: &str) {
    info!("{}", event);
}

pub fn log_error(error: &str) {
    error!("{}", error);
}