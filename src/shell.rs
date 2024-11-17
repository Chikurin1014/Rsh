mod interactor;
mod msg;
mod signal_handler;
mod worker;

use tokio::sync::mpsc::channel;

use crate::shell::{interactor::Interactor, signal_handler::SignalHandler, worker::Worker};

const SHELL_NAME: &str = "rsh";

pub struct Shell {
    interactor: Interactor,
    runner: Worker,
    signal_handler: SignalHandler,
}

impl Shell {
    pub fn new() -> Shell {
        let (interactor_tx, interactor_rx) = channel(100);
        let (runner_tx1, runner_rx) = channel(100);
        let runner_tx2 = runner_tx1.clone();

        let interactor = Interactor::new(interactor_rx, runner_tx1, "rsh.log");
        let runner = Worker::new(runner_rx, interactor_tx);
        let signal_handler = SignalHandler::new(runner_tx2);

        Shell {
            interactor,
            runner,
            signal_handler,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let signal_handler_handle = self.signal_handler.spawn()?;
        let runner_handle = self.runner.spawn();
        let interactor_handle = self.interactor.spawn();

        let (runner_result, interactor_result) =
            tokio::try_join!(runner_handle, interactor_handle)?;

        signal_handler_handle.abort();
        runner_result?;
        interactor_result?;

        Ok(())
    }
}
