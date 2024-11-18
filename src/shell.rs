mod interactor;
mod msg;
mod signal_handler;
mod worker;

use tokio::sync::mpsc::channel;

use crate::shell::{interactor::Interactor, signal_handler::SignalHandler, worker::Worker};

const SHELL_NAME: &str = "rsh";

pub struct Shell {
    interactor: Interactor,
    worker: Worker,
    signal_handler: SignalHandler,
}

impl Shell {
    pub fn new() -> Shell {
        let (interactor_tx, interactor_rx) = channel(100);
        let (worker_tx1, worker_rx) = channel(100);
        let worker_tx2 = worker_tx1.clone();

        let interactor = Interactor::new(interactor_rx, worker_tx1, "rsh.log");
        let worker = Worker::new(worker_rx, interactor_tx);
        let signal_handler = SignalHandler::new(worker_tx2);

        Shell {
            interactor,
            worker,
            signal_handler,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let signal_handler_handle = self.signal_handler.spawn()?;
        let worker_handle = self.worker.spawn();
        let interactor_handle = self.interactor.spawn();

        worker_handle.await??;
        interactor_handle.await??;
        signal_handler_handle.abort();

        Ok(())
    }
}
