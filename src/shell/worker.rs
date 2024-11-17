mod command;
use signal_hook::consts::signal;
use tokio::{sync::mpsc, task::JoinHandle};

use crate::shell::{
    msg::{InteractorMsg, RunnerMsg},
    worker::command::{run_command, Command},
};

pub struct Worker {
    rx: mpsc::Receiver<RunnerMsg>,
    interactor_tx: mpsc::Sender<InteractorMsg>,
}

impl Worker {
    pub fn new(
        rx: mpsc::Receiver<RunnerMsg>,
        interactor_tx: mpsc::Sender<InteractorMsg>,
    ) -> Worker {
        Worker { rx, interactor_tx }
    }

    pub fn spawn(mut self) -> JoinHandle<anyhow::Result<i32>> {
        let handle = tokio::spawn(async move {
            let mut exit_code = 0;
            while let Some(msg) = self.rx.recv().await {
                exit_code = match msg {
                    RunnerMsg::Command { command } => run_command(Command::new(&command)).await?,
                    RunnerMsg::Signal {
                        signal: signal::SIGINT,
                    } => {
                        eprintln!("SIGINT");
                        self.interactor_tx
                            .send(InteractorMsg::Quit { exit_code: 130 })
                            .await?;
                        break;
                    }
                    RunnerMsg::Signal {
                        signal: signal::SIGTSTP,
                    } => {
                        eprintln!("SIGTSTP");
                        self.interactor_tx
                            .send(InteractorMsg::Quit { exit_code: 148 })
                            .await?;
                        break;
                    }
                    RunnerMsg::Signal {
                        signal: signal::SIGCHLD,
                    } => {
                        eprintln!("SIGCHLD");
                        self.interactor_tx
                            .send(InteractorMsg::Continue { exit_code: 0 })
                            .await?;
                        continue;
                    }
                    _ => continue,
                };
            }
            Ok(exit_code)
        });
        handle
    }
}
