use std::process::{Child, Stdio};
use std::time::{Duration, Instant};
use std::thread;

use crate::error::{ServerError, Result};

/// Manages the lifecycle of a child process
pub struct ProcessManager {
    child: Child,
}

impl ProcessManager {
    pub fn spawn(mut command: std::process::Command) -> Result<Self> {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ServerError::ProcessStart(e.to_string()))?;

        Ok(Self { child })
    }

    pub fn take_stdout(&mut self) -> Option<std::process::ChildStdout> {
        self.child.stdout.take()
    }

    pub fn take_stderr(&mut self) -> Option<std::process::ChildStderr> {
        self.child.stderr.take()
    }

    pub fn try_wait(&mut self) -> Result<Option<std::process::ExitStatus>> {
        self.child.try_wait().map_err(ServerError::from)
    }

    pub fn kill_and_wait(&mut self, timeout: Duration) -> Result<()> {
        // Try to kill first
        if let Err(e) = self.child.kill() {
            // If kill fails because process already exited, that's fine; otherwise return error
            match e.kind() {
                std::io::ErrorKind::InvalidInput => {
                    // On Windows, InvalidInput may indicate the process has already exited
                }
                _ => return Err(ServerError::ProcessManagement(e.to_string())),
            }
        }

        let start = Instant::now();
        loop {
            match self.child.try_wait() {
                Ok(Some(_)) => return Ok(()),
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        // Give up waiting
                        return Ok(());
                    }
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }
                Err(e) => return Err(ServerError::ProcessManagement(e.to_string())),
            }
        }
    }
}
