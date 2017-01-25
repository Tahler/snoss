#![allow(dead_code)]
#![allow(unused_must_use)]

#[macro_use]
extern crate enum_primitive;

mod byte_utils;
mod io_utils;
mod shell;
mod system;

use std::io;

use system::system::System;
use shell::shell::Shell;

const PROMPT: &'static str = "> ";

fn main() {
    let system = System::new();

    let prompt = PROMPT.to_string();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut shell = Shell::new(system, prompt, stdin.lock(), stdout);
    let result = shell.start();
    if result.is_err() {
        println!("Err: {:?}", result.err().unwrap());
    }
}
