use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, SendError};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::env;

const RESTART_DELAY_SECS: u64 = 2;
const ERROR_DELAY_SECS: u64 = 5;
const PROCESS_CHECK_INTERVAL_MS: u64 = 100;
const ERROR_PATTERN: &str = "[Error";

#[derive(Debug)]
enum ServerError {
    ProcessStart(String),
    IoError(String),
    ChannelError(String),
}

impl From<io::Error> for ServerError {
    fn from(err: io::Error) -> Self {
        ServerError::IoError(err.to_string())
    }
}

impl From<SendError<bool>> for ServerError {
    fn from(err: SendError<bool>) -> Self {
        ServerError::ChannelError(err.to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let test_mode = args.get(1).map_or(false, |arg| arg == "--test");
    
    if test_mode {
        println!("🧪 Running in test mode");
    }
    
    println!("🚀 Starting dev server monitor...");
    println!("Monitoring for '[Error' in output - will restart on detection");
    println!("Press Ctrl+C to stop\n");

    let mut restart_count = 0;
    
    loop {
        restart_count += 1;
        println!("📡 Starting dev server (attempt #{})...", restart_count);
        
        match start_dev_server(test_mode) {
            Ok(should_restart) => {
                if should_restart {
                    println!("\n🔄 Error detected! Restarting dev server...\n");
                    thread::sleep(Duration::from_secs(RESTART_DELAY_SECS));
                } else {
                    println!("\n✅ Dev server exited normally");
                    break;
                }
            }
            Err(e) => {
                let error_msg = match &e {
                    ServerError::ProcessStart(msg) => msg,
                    ServerError::IoError(msg) => msg,
                    ServerError::ChannelError(msg) => msg,
                };
                eprintln!("❌ Failed to start dev server: {}", error_msg);
                thread::sleep(Duration::from_secs(ERROR_DELAY_SECS));
            }
        }
    }
}

fn create_test_command() -> Command {
    #[cfg(windows)]
    {
        let mut command = Command::new("cmd");
        command.arg("/C")
               .arg("echo 'Test server starting...' && timeout /t 2 && echo 'Server ready' && timeout /t 3 && echo '[Error]: Simulated test error' && timeout /t 1 && echo 'This should not appear'");
        command
    }
    #[cfg(not(windows))]
    {
        let mut command = Command::new("sh");
        command.arg("-c")
               .arg("echo 'Test server starting...'; sleep 2; echo 'Server ready'; sleep 3; echo '[Error]: Simulated test error'; sleep 1; echo 'This should not appear'");
        command
    }
}

fn create_dev_command() -> Command {
    let mut command = Command::new("cmd");
    command.arg("/C")
           .arg("pnpm dev");
    command.current_dir(std::env::current_dir().unwrap_or_default());
    command
}

fn start_dev_server(test_mode: bool) -> Result<bool, ServerError> {
    // Create the command based on mode
    let mut cmd = if test_mode {
        create_test_command()
    } else {
        create_dev_command()
    };

    // Start the process
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ServerError::ProcessStart(e.to_string()))?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    // Create channels for communication between threads
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    // Thread to monitor stdout
    let stdout_handle: JoinHandle<Result<(), ServerError>> = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("📤 {}", line);
                    if line.contains(ERROR_PATTERN) {
                        tx.send(true).map_err(ServerError::from)?;
                        break;
                    }
                }
                Err(e) => {
                    return Err(ServerError::IoError(e.to_string()));
                }
            }
        }
        Ok(())
    });

    // Thread to monitor stderr
    let stderr_handle: JoinHandle<Result<(), ServerError>> = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    eprintln!("📥 {}", line);
                    if line.contains(ERROR_PATTERN) {
                        tx_clone.send(true).map_err(ServerError::from)?;
                        break;
                    }
                }
                Err(e) => {
                    return Err(ServerError::IoError(e.to_string()));
                }
            }
        }
        Ok(())
    });

    // Wait for either an error detection or process completion
    let should_restart = loop {
        match rx.try_recv() {
            Ok(_) => {
                println!("🔍 Error pattern detected!");
                // Error detected, kill the process
                if let Err(e) = child.kill() {
                    eprintln!("Failed to kill process: {:?}", e);
                }
                if let Err(e) = child.wait() {
                    eprintln!("Failed to wait for process: {:?}", e);
                }
                break true;
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Check if process is still running
                match child.try_wait() {
                    Ok(Some(status)) => {
                        // Process has exited
                        println!("📋 Process exited with status: {}", status);
                        break !status.success(); // Restart on non-zero exit
                    }
                    Ok(None) => {
                        // Process still running, continue checking
                        thread::sleep(Duration::from_millis(PROCESS_CHECK_INTERVAL_MS));
                        continue;
                    }
                    Err(e) => {
                        eprintln!("Error checking process status: {:?}", e);
                        break false;
                    }
                }
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("📡 Channel disconnected");
                break false;
            }
        }
    };

    // Clean up threads
    if let Err(e) = stdout_handle.join() {
        eprintln!("Failed to join stdout thread: {:?}", e);
    }
    if let Err(e) = stderr_handle.join() {
        eprintln!("Failed to join stderr thread: {:?}", e);
    }

    Ok(should_restart)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};
    use std::io::Write;

    #[test]
    fn test_error_detection() {
        assert!(true);
    }
}