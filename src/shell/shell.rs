use std::io;
use std::io::BufRead;
use std::io::Write;

use super::cmd::Command;
use super::super::system::system::System;

#[derive(Debug)]
pub struct Shell<R: BufRead, W: Write> {
    system: System,
    // TODO: make it a `&'static str` or `&'a str`?
    prompt: String,
    input: R,
    output: W,
}

impl<R: BufRead, W: Write> Shell<R, W> {
    pub fn new(system: System, prompt: String, input: R, output: W) -> Self {
        Shell {
            system: system,
            prompt: prompt,
            input: input,
            output: output,
        }
    }

    pub fn start(&mut self) {
        loop {
            let cmd = self.get_user_cmd();
            match cmd {
                Command::Exit => break,
                _ => {
                    let output = self.exec_cmd(cmd);
                    self.write_ln(&output);
                }
            }
        }
    }

    fn exec_cmd(&mut self, cmd: Command) -> String {
        use super::cmd::Command::*;

        match cmd {
            ListFiles => self.system.list_files(),
            ProcessStatus => "Not yet implemented.".to_string(),
            Execute => "Not yet implemented.".to_string(),
            ExecuteWithInfo => "Not yet implemented.".to_string(),
            Kill => "Not yet implemented.".to_string(),
            Exit => "Bye!".to_string(),
        }
    }

    fn get_user_cmd(&mut self) -> Command {
        let mut optional_cmd = None;
        while optional_cmd.is_none() {
            self.write_prompt();
            let cmd_str = self.read_line();
            let trimmed = cmd_str.trim();
            optional_cmd = match Command::decode(trimmed) {
                Some(cmd) => Some(cmd),
                None => {
                    self.write_ln(&format!("{}: command not found", trimmed));
                    None
                }
            }
        }
        optional_cmd.unwrap()
    }

    fn read_line(&mut self) -> String {
        let mut input_text = String::new();
        self.input
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let trimmed = input_text.trim();
        trimmed.to_string()
    }

    fn write_prompt(&mut self) {
        let prompt = self.prompt.to_owned();
        self.write(&prompt);
    }

    fn write(&mut self, msg: &str) {
        self.output.write_all(msg.as_bytes());
        self.output.flush();
    }

    fn write_ln(&mut self, msg: &str) {
        let msg_with_ln = msg.to_string() + "\n";
        self.write(&msg_with_ln);
    }
}
