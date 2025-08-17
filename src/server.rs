use std::thread;

use crate::command::{CommandBuilder, CommandType};
use crate::config::Config;
use crate::cli_config::CliConfig;
use crate::error::Result;
use crate::monitor::ProcessMonitor;
use crate::process::ProcessManager;

/// Main server management logic
pub struct DevServer {
    config: Config,
    cli_config: Option<CliConfig>,
    test_mode: bool,
}

impl DevServer {
    pub fn new(config: Config, test_mode: bool) -> Self {
        Self { 
            config, 
            cli_config: None,
            test_mode 
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Load CLI configuration if not in test mode
        if !self.test_mode {
            let cli_config = CliConfig::load_or_create()?;
            // Update the error pattern from CLI config
            self.config.error_pattern = cli_config.error_pattern.clone();
            self.cli_config = Some(cli_config);
        }

        self.print_startup_info();

        let mut restart_count = 0;
        let monitor = ProcessMonitor::new(self.config.clone());

        loop {
            restart_count += 1;
            println!("📡 Starting dev server (attempt #{})...", restart_count);

            match self.start_server_attempt(&monitor) {
                Ok(should_restart) => {
                    if should_restart {
                        println!("\n🔄 Error detected! Restarting dev server...\n");
                        thread::sleep(self.config.restart_delay);
                    } else {
                        println!("\n✅ Dev server exited normally");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to start dev server: {}", e);
                    thread::sleep(self.config.error_delay);
                }
            }
        }

        Ok(())
    }

    fn start_server_attempt(&self, monitor: &ProcessMonitor) -> Result<bool> {
        let command_type = if self.test_mode {
            CommandType::Test
        } else {
            CommandType::Dev(self.cli_config.as_ref().unwrap().clone())
        };

        let command = CommandBuilder::build(command_type);
        let process = ProcessManager::spawn(command)?;
        monitor.monitor(process)
    }

    fn print_startup_info(&self) {
        if self.test_mode {
            println!("🧪 Running in test mode");
        } else if let Some(cli_config) = &self.cli_config {
            println!("🚀 Starting dev server monitor for: {}", cli_config.run_command);
        } else {
            println!("🚀 Starting dev server monitor...");
        }

        println!(
            "Monitoring for '{}' in output - will restart on detection",
            self.config.error_pattern
        );
        println!("Press Ctrl+C to stop\n");
    }
}
