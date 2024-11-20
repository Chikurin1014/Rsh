mod command;
use signal_hook::consts::signal;
use tokio::{sync::mpsc, task::JoinHandle};

use crate::shell::{
    msg::{InteractorMsg, WorkerMsg},
    worker::command::{run_command, Command},
};

use super::msg::CommandStatus;

pub struct Worker {
    rx: mpsc::Receiver<WorkerMsg>,
    interactor_tx: mpsc::Sender<InteractorMsg>,
}

impl Worker {
    pub fn new(
        rx: mpsc::Receiver<WorkerMsg>,
        interactor_tx: mpsc::Sender<InteractorMsg>,
    ) -> Worker {
        Worker { rx, interactor_tx }
    }

    pub fn spawn(mut self) -> JoinHandle<anyhow::Result<i32>> {
        let handle = tokio::spawn(async move {
            let mut exit_code = 0;
            let mut status = CommandStatus::Success;
            while let Some(msg) = self.rx.recv().await {
                println!("Worker received: {:?}", msg);
                let continue_loop = match msg {
                    WorkerMsg::Command(command) => {
                        exit_code = run_command(Command::new(&command)).await?;
                        status = exit_code.into();
                        true
                    }
                    WorkerMsg::Signal(signal::SIGINT) => {
                        eprintln!("SIGINT");
                        exit_code = 130;
                        false
                    }
                    WorkerMsg::Signal(signal::SIGTSTP) => {
                        eprintln!("SIGTSTP");
                        exit_code = 148;
                        false
                    }
                    WorkerMsg::Signal(signal::SIGCHLD) => {
                        eprintln!("SIGCHLD");
                        exit_code = 0;
                        true
                    }
                    _ => continue,
                }; // match msg
                if continue_loop {
                    self.interactor_tx
                        .send(InteractorMsg::Continue(status))
                        .await?;
                } else {
                    self.interactor_tx.send(InteractorMsg::Quit).await?;
                    break; // get out of the while loop
                }
            } // while loop
            Ok(exit_code)
        });
        handle
    }
}
