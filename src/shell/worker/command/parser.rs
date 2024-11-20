use super::{BuiltinCommand, BuiltinCommandType, Command, TokioCommand};

pub fn parse(command: &str) -> Command {
    parse_builtin(command)
        .map(|cmd| Command::Builtin(cmd))
        .unwrap_or({
            let mut parts = command.split_whitespace();
            let cmd_part = {
                let a = parts.next().unwrap_or_default();
                if a.starts_with('^') {
                    &a[1..]
                } else {
                    a
                }
            };
            let arg_parts = parts.map(|s| s.to_string()).collect::<Vec<_>>();
            let mut cmd = TokioCommand::new(cmd_part);
            cmd.args(arg_parts);
            Command::External(cmd)
        })
}

fn parse_builtin(command: &str) -> Option<BuiltinCommand> {
    let mut parts = command.split_whitespace();
    let cmd = parts.next()?;
    let args = parts.map(|s| s.to_string()).collect();
    match cmd {
        "exit" => Some(BuiltinCommand {
            cmd_type: BuiltinCommandType::Exit,
            args,
        }),
        "cd" => {
            if args.len() > 1 {
                return None;
            }
            Some(BuiltinCommand {
                cmd_type: BuiltinCommandType::Cd,
                args,
            })
        }
        "pwd" => {
            if !args.is_empty() {
                return None;
            }
            Some(BuiltinCommand {
                cmd_type: BuiltinCommandType::Pwd,
                args,
            })
        }
        "ls" => {
            if args.len() > 1 {
                return None;
            }
            Some(BuiltinCommand {
                cmd_type: BuiltinCommandType::Ls,
                args,
            })
        }
        "echo" => {
            if args.is_empty() {
                return None;
            }
            if args[0].starts_with('-') {
                return None;
            }
            Some(BuiltinCommand {
                cmd_type: BuiltinCommandType::Echo,
                args,
            })
        }
        _ => None,
    }
}
