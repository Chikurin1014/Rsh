use std::io::Error;

use signal_hook::{consts::signal, iterator::Signals};
use tokio::sync::mpsc::Sender;

use crate::shell::msg::WorkerMsg;

pub struct SignalHandler {
    worker_tx: Sender<WorkerMsg>,
}

impl SignalHandler {
    pub fn new(runner_tx: Sender<WorkerMsg>) -> SignalHandler {
        SignalHandler {
            worker_tx: runner_tx,
        }
    }

    pub fn spawn(self) -> Result<tokio::task::JoinHandle<anyhow::Result<()>>, Error> {
        let mut signals = Signals::new(&[signal::SIGINT, signal::SIGTSTP, signal::SIGCHLD])?;
        let handle = tokio::spawn(async move {
            for signal in signals.forever() {
                self.worker_tx.send(WorkerMsg::Signal { signal }).await?;
            }
            Ok(())
        });
        Ok(handle)
    }
}
