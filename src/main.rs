#![allow(dead_code)]
#![allow(unused_must_use)]

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate time;

mod byte_utils;
mod io_utils;
mod time_utils;
mod sh;
mod os;

use os::System;
use sh::Shell;

const PROMPT: &'static str = "> ";

fn main() {
    // Init logging
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    info!("Booting...");

    let system = System::init();

    let prompt = PROMPT.to_string();

    let mut shell = Shell::new(system, prompt);
    let result = shell.start();
    if result.is_err() {
        println!("Err: {:?}", result.err().unwrap());
    }
    info!("Shutting down...");
}
