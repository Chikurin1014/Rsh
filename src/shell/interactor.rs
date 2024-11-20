use nix::sys::signal::{signal, SigHandler, Signal};
use rustyline;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

use crate::shell::{
    msg::{InteractorMsg, WorkerMsg},
    SHELL_NAME,
};

use super::msg::CommandStatus;

pub struct Interactor {
    rx: Receiver<InteractorMsg>,
    worker_tx: Sender<WorkerMsg>,
    logfile_path: String,
}

impl Interactor {
    pub fn new(
        rx: Receiver<InteractorMsg>,
        worker_tx: Sender<WorkerMsg>,
        logfile_path: &str,
    ) -> Interactor {
        Interactor {
            rx,
            worker_tx,
            logfile_path: logfile_path.to_string(),
        }
    }

    pub fn spawn(mut self) -> JoinHandle<anyhow::Result<()>> {
        let handle = tokio::spawn(async move {
            // SIGTTOUを無視しないとSIGTSTPが送信され、プロセスが停止する
            unsafe { signal(Signal::SIGTTOU, SigHandler::SigIgn) }?;
            let mut rl = rustyline::DefaultEditor::new()?;
            let mut previous_status = CommandStatus::Success;
            loop {
                let line_start_with = if previous_status == CommandStatus::Success {
                    "-->"
                } else {
                    "-?>"
                };
                match rl.readline(&format!("{} {} ", SHELL_NAME, line_start_with)) {
                    Ok(line) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        rl.add_history_entry(trimmed)?;

                        self.worker_tx
                            .send(WorkerMsg::Command(trimmed.to_string()))
                            .await?;
                        match self.rx.recv().await {
                            Some(InteractorMsg::Continue(status)) => {
                                previous_status = status;
                                continue;
                            }
                            Some(InteractorMsg::Quit) => {
                                break;
                            }
                            None => {
                                break;
                            }
                        }
                    } // Ok(line)
                    Err(rustyline::error::ReadlineError::Interrupted) => continue,
                    Err(rustyline::error::ReadlineError::Eof) => {
                        eprintln!("EOF");
                        self.worker_tx
                            .send(WorkerMsg::Command("exit".to_string()))
                            .await?;
                        if let Some(InteractorMsg::Quit) = self.rx.recv().await {
                            break;
                        } else {
                            return Err(anyhow::anyhow!("Failed to receive InteractorMsg::Quit"));
                        }
                    }
                    Err(e) => return Err(anyhow::anyhow!(e)),
                } // match rl.readline
            } // loop
            rl.save_history(&self.logfile_path)?;
            Ok(())
        }); // handle
        handle
    }
}
