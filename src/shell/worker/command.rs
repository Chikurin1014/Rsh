mod parser;
mod runner;

use tokio::process::Command as TokioCommand;

use parser::parse;

pub use runner::run_command;

pub enum Command {
    Builtin(BuiltinCommand),
    External(TokioCommand),
}

impl Command {
    pub fn new(command: &str) -> Command {
        parse(command)
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
