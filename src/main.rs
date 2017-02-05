#![allow(dead_code)]
#![allow(unused_must_use)]

#[macro_use]
extern crate enum_primitive;

mod byte_utils;
mod io_utils;
mod sh;
mod os;

use os::System;
use sh::Shell;

const PROMPT: &'static str = "> ";

fn main() {
    let system = System::new();

    let prompt = PROMPT.to_string();

    let mut shell = Shell::new(system, prompt);
    let result = shell.start();
    if result.is_err() {
        println!("Err: {:?}", result.err().unwrap());
    }
}
