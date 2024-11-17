use std::io::Error;

use signal_hook::{consts::signal, iterator::Signals};
use tokio::sync::mpsc::Sender;

use crate::shell::msg::RunnerMsg;

pub struct SignalHandler {
    runner_tx: Sender<RunnerMsg>,
}

impl SignalHandler {
    pub fn new(runner_tx: Sender<RunnerMsg>) -> SignalHandler {
        SignalHandler { runner_tx }
    }

    pub fn spawn(self) -> Result<tokio::task::JoinHandle<anyhow::Result<()>>, Error> {
        let mut signals = Signals::new(&[signal::SIGINT, signal::SIGTSTP, signal::SIGCHLD])?;
        let handle = tokio::spawn(async move {
            for signal in signals.forever() {
                eprintln!("SignalHandler: {:?}", signal);
                self.runner_tx.send(RunnerMsg::Signal { signal }).await?;
            }
            Ok(())
        });
        Ok(handle)
    }
}
