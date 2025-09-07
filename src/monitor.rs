use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::config::Config;
use crate::error::{Result, ServerError};
use crate::process::ProcessManager;

/// Messages passed between monitoring threads and the main loop
#[derive(Debug)]
pub enum WatchMessage {
    ErrorDetected,
    IoError(String),
}

/// Monitors a process for error patterns and manages its lifecycle
pub struct ProcessMonitor {
    config: Config,
}

impl ProcessMonitor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn monitor(&self, mut process: ProcessManager) -> Result<bool> {
        let stdout = process.take_stdout().expect("Failed to capture stdout");
        let stderr = process.take_stderr().expect("Failed to capture stderr");

        // Create channels for communication between threads
        let (tx, rx) = mpsc::channel::<WatchMessage>();
        let tx_stdout = tx.clone();
        let tx_stderr = tx.clone();

        // Start monitoring threads
        let stdout_handle = self.spawn_stdout_monitor(stdout, tx_stdout);
        let stderr_handle = self.spawn_stderr_monitor(stderr, tx_stderr);

        // Wait for either an error detection or process completion
        let should_restart = self.wait_for_completion(&mut process, rx)?;

        // Clean up threads
        self.cleanup_threads(stdout_handle, stderr_handle);

        Ok(should_restart)
    }

    fn spawn_stdout_monitor(
        &self,
        stdout: std::process::ChildStdout,
        tx: Sender<WatchMessage>,
    ) -> JoinHandle<Result<()>> {
        let error_pattern = self.config.error_pattern.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("📤 {}", line);
                        if line.contains(&error_pattern) {
                            tx.send(WatchMessage::ErrorDetected)?;
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(WatchMessage::IoError(e.to_string()));
                        return Err(ServerError::IoError(e.to_string()));
                    }
                }
            }
            Ok(())
        })
    }

    fn spawn_stderr_monitor(
        &self,
        stderr: std::process::ChildStderr,
        tx: Sender<WatchMessage>,
    ) -> JoinHandle<Result<()>> {
        let error_pattern = self.config.error_pattern.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        eprintln!("📥 {}", line);
                        if line.contains(&error_pattern) {
                            tx.send(WatchMessage::ErrorDetected)?;
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(WatchMessage::IoError(e.to_string()));
                        return Err(ServerError::IoError(e.to_string()));
                    }
                }
            }
            Ok(())
        })
    }

    fn wait_for_completion(
        &self,
        process: &mut ProcessManager,
        rx: Receiver<WatchMessage>,
    ) -> Result<bool> {
        loop {
            match rx.recv_timeout(self.config.process_check_interval) {
                Ok(WatchMessage::ErrorDetected) => {
                    println!("🔍 Error pattern detected!");
                    if let Err(e) = process.kill_and_wait(self.config.shutdown_timeout) {
                        eprintln!("Failed to stop process cleanly: {}", e);
                    }
                    return Ok(true);
                }
                Ok(WatchMessage::IoError(msg)) => {
                    eprintln!("Reader IO error: {}", msg);
                    let _ = process.kill_and_wait(self.config.shutdown_timeout);
                    return Ok(true); // treat IO errors as reason to restart
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Check if process exited
                    match process.try_wait()? {
                        Some(status) => {
                            println!("📋 Process exited with status: {}", status);
                            return Ok(!status.success()); // Restart on non-zero exit
                        }
                        None => continue, // Still running
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    println!("📡 Channel disconnected");
                    return Ok(false);
                }
            }
        }
    }

    fn cleanup_threads(
        &self,
        stdout_handle: JoinHandle<Result<()>>,
        stderr_handle: JoinHandle<Result<()>>,
    ) {
        match stdout_handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => eprintln!("stdout thread returned error: {}", e),
            Err(panic) => eprintln!("stdout thread panicked: {:?}", panic),
        }
        match stderr_handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => eprintln!("stderr thread returned error: {}", e),
            Err(panic) => eprintln!("stderr thread panicked: {:?}", panic),
        }
    }
}
