#[derive(Debug)]
pub struct CommandWithArgs {
    pub cmd: Command,
    pub args: Option<Vec<String>>,
}

impl CommandWithArgs {
    pub fn from_string(s: &str) -> Option<Self> {
        if s.is_empty() {
            None
        } else {
            let tokens: Vec<&str> = s.split_whitespace().collect();
            let cmd = extract_cmd(&tokens);
            cmd.map(|cmd| {
                let args = extract_args(&tokens);
                CommandWithArgs {
                    cmd: cmd,
                    args: args,
                }
            })
        }
    }
}

#[derive(Debug)]
pub enum Command {
    ListFiles,
    ProcessStatus,
    Execute,
    ExecuteAsync,
    Kill,
    Exit,
}

fn extract_cmd(tokens: &Vec<&str>) -> Option<Command> {
    use self::Command::*;

    let cmd = tokens[0];
    match cmd {
        "ls" => Some(ListFiles),
        "ps" => Some(ProcessStatus),
        "exec" => {
            if *tokens.last().unwrap() == "&" {
                Some(Execute)
            } else {
                Some(ExecuteAsync)
            }
        },
        "kill" => Some(Kill),
        "exit" => Some(Exit),
        _ => None,
    }
}

fn extract_args(tokens: &Vec<&str>) -> Option<Vec<String>> {
    if tokens.len() == 0 {
        None
    } else {
        let range = if *tokens.last().unwrap() == "&" {
            1..(tokens.len() - 1)
        } else {
            1..(tokens.len())
        };
        let args = tokens[range].iter().map(|ref_str| ref_str.to_string()).collect();
        Some(args)
    }
}
