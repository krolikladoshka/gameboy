use logging::init_logger;

pub mod logging;
pub mod cpu;
pub mod core;
pub mod bus;
pub mod cartridge;


fn main() {
    match init_logger() {
        Ok(_) => {},
        Err(error) => {
            panic!("Could not setup logger {}", error);
        },
    }


}