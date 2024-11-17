use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuiltinCommandError {
    #[error("failed to parse args: {args}")]
    ParseArgsError { args: String },
    #[error("failed to execute command {command}: {error}")]
    CommandExecutionError { command: String, error: String },
}

pub fn run_exit(args: &[&str]) -> anyhow::Result<i32> {
    let exit_code = if args.is_empty() {
        0
    } else {
        let args = args.join(" ");
        args.parse()
            .map_err(|_| BuiltinCommandError::ParseArgsError { args })?
    };
    std::process::exit(exit_code);
}

pub fn run_cd(args: &[&str]) -> anyhow::Result<i32> {
    let path = if args.is_empty() {
        std::env::var("HOME").map_err(|_| BuiltinCommandError::ParseArgsError {
            args: "".to_string(),
        })?
    } else {
        args.join(" ")
    };
    std::env::set_current_dir(&path).map_err(|_| BuiltinCommandError::CommandExecutionError {
        command: "cd".to_string(),
        error: format!("failed to change directory to {}", path),
    })?;
    Ok(0)
}

pub fn run_pwd(_args: &[&str]) -> anyhow::Result<i32> {
    let path = std::env::current_dir().map_err(|_| BuiltinCommandError::CommandExecutionError {
        command: "pwd".to_string(),
        error: "failed to get current directory".to_string(),
    })?;
    println!("{}", path.display());
    Ok(0)
}

pub fn run_ls(args: &[&str]) -> anyhow::Result<i32> {
    let path = if args.is_empty() {
        std::env::current_dir().map_err(|_| BuiltinCommandError::CommandExecutionError {
            command: "ls".to_string(),
            error: "failed to get current directory".to_string(),
        })?
    } else {
        std::path::PathBuf::from(args[0])
    };
    let entries =
        std::fs::read_dir(path).map_err(|_| BuiltinCommandError::CommandExecutionError {
            command: "ls".to_string(),
            error: "failed to read directory".to_string(),
        })?;
    for entry in entries {
        let entry = entry.map_err(|_| BuiltinCommandError::CommandExecutionError {
            command: "ls".to_string(),
            error: "failed to read entry".to_string(),
        })?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(0)
}

pub fn run_echo(args: &[&str]) -> anyhow::Result<i32> {
    println!("{}", args.join(" "));
    Ok(0)
}
