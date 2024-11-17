use nix::sys::signal::{signal, SigHandler, Signal};
use rustyline;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinHandle,
};

use crate::shell::{
    msg::{InteractorMsg, RunnerMsg},
    SHELL_NAME,
};

pub struct Interactor {
    rx: Receiver<InteractorMsg>,
    runner_tx: Sender<RunnerMsg>,
    logfile_path: String,
}

impl Interactor {
    pub fn new(
        rx: Receiver<InteractorMsg>,
        runner_tx: Sender<RunnerMsg>,
        logfile_path: &str,
    ) -> Interactor {
        Interactor {
            rx,
            runner_tx,
            logfile_path: logfile_path.to_string(),
        }
    }

    pub fn spawn(mut self) -> JoinHandle<anyhow::Result<i32>> {
        let handle = tokio::spawn(async move {
            // SIGTTOUを無視しないとSIGTSTPが送信され、プロセスが停止する
            unsafe { signal(Signal::SIGTTOU, SigHandler::SigIgn) }?;
            let mut rl = rustyline::DefaultEditor::new()?;
            let mut previous_exit_code = 0;
            let exit_code = loop {
                let line_start_with = if previous_exit_code == 0 {
                    "-->"
                } else {
                    "-?>"
                };
                match rl.readline(&format!("{} {}", SHELL_NAME, line_start_with)) {
                    Ok(line) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        rl.add_history_entry(trimmed)?;

                        self.runner_tx
                            .send(RunnerMsg::Command {
                                command: trimmed.to_string(),
                            })
                            .await?;
                        match self.rx.recv().await {
                            Some(InteractorMsg::Continue { exit_code }) => {
                                previous_exit_code = exit_code;
                            }
                            Some(InteractorMsg::Quit { exit_code }) => {
                                break exit_code;
                            }
                            None => {
                                break previous_exit_code;
                            }
                        }
                    } // Ok(line)
                    Err(rustyline::error::ReadlineError::Interrupted) => continue,
                    Err(rustyline::error::ReadlineError::Eof) => {
                        self.runner_tx
                            .send(RunnerMsg::Command {
                                command: "exit".to_string(),
                            })
                            .await?;
                        if let Some(InteractorMsg::Quit { exit_code }) = self.rx.recv().await {
                            break exit_code;
                        } else {
                            return Err(anyhow::anyhow!("Failed to receive InteractorMsg::Quit"));
                        }
                    }
                    Err(e) => return Err(anyhow::anyhow!(e)),
                } // match rl.readline
            }; // loop
            rl.save_history(&self.logfile_path)?;
            Ok(exit_code)
        }); // handle
        handle
    }
}
