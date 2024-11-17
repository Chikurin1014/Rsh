use super::{BuiltinCommand, BuiltinCommandType};

pub fn parse_builtin(command: &str) -> Option<BuiltinCommand> {
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
