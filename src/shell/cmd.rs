#[derive(Debug, Clone)]
pub struct CommandWithArgs {
    pub cmd: Command,
    pub args: Vec<String>,
}

impl CommandWithArgs {
    pub fn from_string(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() == 0 {
            None
        } else {
            let cmd_str = words[0];
            match Command::decode(cmd_str) {
                Some(cmd) => {
                    let args = words[1..].iter().map(|ref_str| ref_str.to_string()).collect();
                    let cmd_args = CommandWithArgs {
                        cmd: cmd,
                        args: args,
                    };
                    Some(cmd_args)
                },
                None => None,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    ListFiles,
    ProcessStatus,
    Execute,
    ExecuteWithInfo,
    Kill,
    Exit,
}

impl Command {
    pub fn decode(cmd: &str) -> Option<Self> {
        use self::Command::*;

        match cmd {
            "ls" => Some(ListFiles),
            "ps" => Some(ProcessStatus),
            "exec" => Some(Execute),
            "exec_i" => Some(ExecuteWithInfo),
            "kill" => Some(Kill),
            "exit" => Some(Exit),
            _ => None,
        }
    }
}
