use io_utils;

use super::cmd::{CommandWithArgs, Command};
use super::super::system::system::System;

#[derive(Debug)]
pub struct Shell {
    system: System,
    // TODO: make it a `&'static str` or `&'a str`? or AsRef<String>
    prompt: String,
}

impl Shell {
    pub fn new(system: System, prompt: String) -> Self {
        Shell {
            system: system,
            prompt: prompt,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        loop {
            let cmd_args = self.get_user_cmd();
            let cmd = &cmd_args.cmd;
            match *cmd {
                Command::Exit => return Ok(()),
                _ => {
                    let result = self.exec_cmd(&cmd_args);
                    let unwrapped = match result {
                        Ok(s) => s,
                        Err(s) => s,
                    };
                    io_utils::write_ln(&unwrapped);
                }
            }
        }
    }

    fn exec_cmd(&mut self, command: &CommandWithArgs) -> Result<String, String> {
        use super::cmd::Command::*;

        match command.cmd {
            ListFiles => Ok(self.system.list_files()),
            ProcessStatus => Ok("Not yet implemented.".to_string()),
            Execute => self.system.exec(&command.args[0], false),
            ExecuteWithInfo => self.system.exec(&command.args[0], true),
            Kill => Ok("Not yet implemented.".to_string()),
            Exit => Ok("Bye!".to_string()),
        }
    }

    fn get_user_cmd(&mut self) -> CommandWithArgs {
        let mut optional_cmd = None;
        while optional_cmd.is_none() {
            self.write_prompt();
            let line = io_utils::read_line();
            optional_cmd = if line.is_empty() {
                None
            } else {
                match CommandWithArgs::from_string(&line) {
                    Some(cmd) => Some(cmd),
                    None => {
                        let first_word = line.split_whitespace().next().unwrap();
                        io_utils::write_ln(&format!("{}: command not found", first_word));
                        None
                    }
                }
            }
        }
        optional_cmd.unwrap()
    }

    fn write_prompt(&self) {
        let prompt = self.prompt.to_owned();
        io_utils::write(&prompt);
    }
}
