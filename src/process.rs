use std::process::{Child, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{Result, ServerError};

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

    pub fn spawn_with_pid_handle(
        mut command: std::process::Command,
        pid_handle: std::sync::Arc<std::sync::Mutex<Option<u32>>>,
    ) -> Result<Self> {
        let child = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ServerError::ProcessStart(e.to_string()))?;
        // Set the PID in the Arc
        {
            let mut pid_lock = pid_handle.lock().unwrap();
            *pid_lock = Some(child.id());
        }
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
        println!("🛑 Terminating process...");

        // On Windows, try to terminate the process tree
        #[cfg(windows)]
        {
            let pid = self.child.id();
            // Use taskkill to terminate the process tree
            let _ = std::process::Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .output();
        }

        // Try to kill the direct child process
        if let Err(e) = self.child.kill() {
            // If kill fails because process already exited, that's fine; otherwise return error
            match e.kind() {
                std::io::ErrorKind::InvalidInput => {
                    // On Windows, InvalidInput may indicate the process has already exited
                    println!("Process already exited");
                }
                _ => {
                    eprintln!("Failed to kill process: {}", e);
                    return Err(ServerError::ProcessManagement(e.to_string()));
                }
            }
        }

        let start = Instant::now();
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    println!("✅ Process terminated with status: {}", status);
                    return Ok(());
                }
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        println!("⚠️ Process didn't terminate within timeout, giving up");
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
