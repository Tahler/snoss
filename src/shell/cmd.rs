pub enum Command {
    ListFiles,
    ProcessStatus,
    Execute,
    ExecuteWithInfo,
    Kill,
    Exit,
}

impl Command {
    pub fn decode(cmd: &str) -> Option<Command> {
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
