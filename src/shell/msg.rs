#[derive(Debug)]
pub enum RunnerMsg {
    Signal { signal: i32 },
    Command { command: String },
}

#[derive(Debug)]
pub enum InteractorMsg {
    Continue { exit_code: i32 },
    Quit { exit_code: i32 },
}
