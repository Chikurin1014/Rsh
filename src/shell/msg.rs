#[derive(Debug)]
pub enum WorkerMsg {
    Signal(i32),
    Command(String),
}

#[derive(Debug)]
pub enum InteractorMsg {
    Continue(CommandStatus),
    Quit,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandStatus {
    Success,
    Failure,
}

impl From<bool> for CommandStatus {
    fn from(b: bool) -> Self {
        if b {
            CommandStatus::Success
        } else {
            CommandStatus::Failure
        }
    }
}

impl From<i32> for CommandStatus {
    fn from(i: i32) -> Self {
        if i == 0 {
            CommandStatus::Success
        } else {
            CommandStatus::Failure
        }
    }
}

impl Into<bool> for CommandStatus {
    fn into(self) -> bool {
        match self {
            CommandStatus::Success => true,
            CommandStatus::Failure => false,
        }
    }
}
