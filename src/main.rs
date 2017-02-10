#![allow(dead_code)]
#![allow(unused_must_use)]

#[macro_use]
extern crate enum_primitive;
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
    use std::io;

    let system = System::init();

    let prompt = PROMPT.to_string();

    let mut shell = Shell::new(system, prompt, io::stdin(), io::stdout());
    let result = shell.start();
    if result.is_err() {
        println!("Err: {:?}", result.err().unwrap());
    }
}
