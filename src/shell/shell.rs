use std::io::BufRead;
use std::io::Write;

use super::cmd::{CommandWithArgs, Command};
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

    pub fn start(&mut self) -> Result<(), String> {
        loop {
            let cmd_args = self.get_user_cmd();
            let cmd = &cmd_args.cmd;
            match *cmd {
                Command::Exit => return Ok(()),
                _ => {
                    let output = self.exec_cmd(&cmd_args)?;
                    self.write_ln(&output);
                }
            }
        }
    }

    fn exec_cmd(&mut self, command: &CommandWithArgs) -> Result<String, String> {
        use super::cmd::Command::*;

        match command.cmd {
            ListFiles => Ok(self.system.list_files()),
            ProcessStatus => Ok("Not yet implemented.".to_string()),
            Execute => self.system.exec(&command.args[0]),
            ExecuteWithInfo => Ok("Not yet implemented.".to_string()),
            Kill => Ok("Not yet implemented.".to_string()),
            Exit => Ok("Bye!".to_string()),
        }
    }

    fn get_user_cmd(&mut self) -> CommandWithArgs {
        let mut optional_cmd = None;
        while optional_cmd.is_none() {
            self.write_prompt();
            let line = self.read_line();
            optional_cmd = if line.is_empty() {
                None
            } else {
                match CommandWithArgs::from_string(&line) {
                    Some(cmd) => Some(cmd),
                    None => {
                        let first_word = line.split_whitespace().next().unwrap();
                        self.write_ln(&format!("{}: command not found", first_word));
                        None
                    }
                }
            }
        }
        optional_cmd.unwrap()
    }

    /// Returns a trimmed line read from the input.
    fn read_line(&mut self) -> String {
        let mut input_text = String::new();
        self.input
            .read_line(&mut input_text)
            .expect("failed to read from input");
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
