mod parser;
mod runner;

use tokio::process::Command as TokioCommand;

use parser::parse_builtin;

pub use runner::run_command;

pub enum Command {
    Builtin(BuiltinCommand),
    External(TokioCommand),
}

impl Command {
    pub fn new(command: &str) -> Command {
        parse_builtin(command)
            .map(|cmd| Command::Builtin(cmd))
            .unwrap_or_else(|| Command::External(TokioCommand::new(command)))
    }
}

pub struct BuiltinCommand {
    pub cmd_type: BuiltinCommandType,
    pub args: Vec<String>,
}

pub enum BuiltinCommandType {
    Exit,
    Cd,
    Pwd,
    Ls,
    Echo,
}
