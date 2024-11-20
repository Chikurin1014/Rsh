mod implement;

use super::{
    Command, {BuiltinCommand, BuiltinCommandType},
};
use anyhow::Context as _;
use implement::{run_cd, run_echo, run_exit, run_ls, run_pwd};

pub async fn run_command(command: Command) -> anyhow::Result<i32> {
    match command {
        Command::Builtin(cmd) => run_builtin(cmd),
        Command::External(mut cmd) => {
            let mut child = cmd.spawn()?;
            let status = child.wait().await?;
            status
                .code()
                .with_context(|| format!("command {:?} exited with no status", cmd))
        }
    }
}

fn run_builtin(cmd: BuiltinCommand) -> anyhow::Result<i32> {
    let args = cmd.args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    match cmd.cmd_type {
        BuiltinCommandType::Exit => run_exit(&args),
        BuiltinCommandType::Cd => run_cd(&args),
        BuiltinCommandType::Pwd => run_pwd(&args),
        BuiltinCommandType::Ls => run_ls(&args),
        BuiltinCommandType::Echo => run_echo(&args),
    }
}
