mod interactor;
mod msg;
mod signal_handler;
mod worker;

use tokio::{
    sync::mpsc::{channel, Sender},
    task::JoinHandle,
};

use crate::shell::{interactor::Interactor, signal_handler::SignalHandler, worker::Worker};
use msg::WorkerMsg;

const SHELL_NAME: &str = "rsh";

pub struct Shell {
    interactor: Interactor,
    worker: Worker,
    signal_handler: SignalHandler,
    worker_tx: Sender<WorkerMsg>,
}

impl Shell {
    pub fn new() -> Shell {
        let (interactor_tx, interactor_rx) = channel(100);
        let (worker_tx1, worker_rx) = channel(100);
        let worker_tx2 = worker_tx1.clone();
        let worker_tx3 = worker_tx1.clone();

        let interactor = Interactor::new(
            interactor_rx,
            worker_tx1,
            format!("$HOME/.{}_history", SHELL_NAME).as_str(),
        );
        let worker = Worker::new(worker_rx, interactor_tx);
        let signal_handler = SignalHandler::new(worker_tx2);

        Shell {
            interactor,
            worker,
            signal_handler,
            worker_tx: worker_tx3,
        }
    }

    pub fn spawn(self) -> ShellHandler {
        let (shell_tx, mut rx) = channel(100);
        let worker_tx = self.worker_tx.clone();

        let handle = tokio::spawn(async move {
            let h = self.run();
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    ShellMsg::Command(command) => {
                        worker_tx.send(WorkerMsg::Command { command }).await?;
                    }
                    ShellMsg::Close => {
                        worker_tx
                            .send(WorkerMsg::Command {
                                command: "exit".to_string(),
                            })
                            .await?;
                        break;
                    }
                }
            }
            h.await
        });
        ShellHandler {
            shell_handle: handle,
            shell_tx,
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

pub enum ShellMsg {
    Command(String),
    Close,
}

pub struct ShellHandler {
    shell_handle: JoinHandle<anyhow::Result<()>>,
    shell_tx: Sender<ShellMsg>,
}

impl ShellHandler {
    pub async fn command(self, command: &str) -> anyhow::Result<Self> {
        self.shell_tx
            .send(ShellMsg::Command(command.to_string()))
            .await?;
        Ok(self)
    }
    pub async fn close(self) -> anyhow::Result<()> {
        self.shell_tx.send(ShellMsg::Close).await?;
        self.shell_handle.await?
    }
}
