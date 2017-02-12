use std::io;
use io_utils;
use sh::cmd::{CommandWithArgs, Command};
use os::System;

// #[derive(Debug)]
pub struct Shell {
    system: System,
    // TODO: make it a `&'static str` or `&'a str`? or AsRef<String>
    prompt: String,
    reader: io::Stdin,
    writer: io::Stdout,
}

impl Shell {
    pub fn new(system: System, prompt: String) -> Self {
        Shell {
            system: system,
            prompt: prompt,
            reader: io::stdin(),
            writer: io::stdout(),
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        loop {
            let cmd_args = self.get_user_cmd();
            let cmd = &cmd_args.cmd;
            match *cmd {
                Command::Exit => return Ok(()),
                Command::Execute => {
                    let file = &cmd_args.args[0];
                    let handle = self.system.exec(file, true)?;
                    handle.join();
                }
                Command::ExecuteAsync => {
                    let file = &cmd_args.args[0];
                    self.system.exec(file, false)?;
                }
                _ => {
                    let result = self.exec_cmd(&cmd_args);
                    let unwrapped = match result {
                        Ok(s) => s,
                        Err(s) => s,
                    };
                    self.write_ln(&unwrapped);
                }
            }
        }
    }

    fn exec_cmd(&mut self, command: &CommandWithArgs) -> Result<String, String> {
        use super::cmd::Command::*;

        match command.cmd {
            ListFiles => Ok(self.system.list_files()),
            ListProcesses => Ok(self.system.list_procs()),
            Kill => {
                let pid = &command.args[0];
                let pid = pid.parse::<u16>().unwrap();
                self.system.kill(pid)?;
                Ok(format!("Killed process {}", pid))
            }
            _ => unreachable!(),
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

    fn read_line(&mut self) -> String {
        io_utils::read_line(&mut self.reader)
    }

    fn write_prompt(&mut self) {
        let prompt = self.prompt.to_owned();
        self.write(&prompt);
    }

    fn write(&mut self, msg: &str) {
        io_utils::write(&mut self.writer, msg)
    }

    fn write_ln(&mut self, msg: &str) {
        io_utils::write_ln(&mut self.writer, msg)
    }
}
