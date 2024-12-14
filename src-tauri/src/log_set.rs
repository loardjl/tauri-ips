use log4rs;

pub fn main() {
    log4rs::init_file("config/log.yml", Default::default()).unwrap();
}
