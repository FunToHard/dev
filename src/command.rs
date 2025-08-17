use std::process::Command;
use std::env;

use crate::cli_config::CliConfig;

/// Represents different types of commands that can be executed
pub enum CommandType {
    Test,
    Dev(CliConfig),
}

/// Command builder for creating process commands
pub struct CommandBuilder;

impl CommandBuilder {
    pub fn build(command_type: CommandType) -> Command {
        match command_type {
            CommandType::Test => Self::create_test_command(),
            CommandType::Dev(config) => Self::create_dev_command(config),
        }
    }

    #[cfg(windows)]
    fn create_test_command() -> Command {
        let mut command = Command::new("cmd");
        command.arg("/C").arg(
            "echo Test server starting... && timeout /t 2 && echo Server ready && timeout /t 3 && echo [Error]: Simulated test error && timeout /t 1 && echo This should not appear"
        );
        command
    }

    #[cfg(not(windows))]
    fn create_test_command() -> Command {
        let mut command = Command::new("sh");
        command.arg("-c").arg(
            "echo 'Test server starting...'; sleep 2; echo 'Server ready'; sleep 3; echo '[Error]: Simulated test error'; sleep 1; echo 'This should not appear'"
        );
        command
    }

    #[cfg(windows)]
    fn create_dev_command(config: CliConfig) -> Command {
        let mut command = Command::new("cmd");
        // Use /C to run command and return, but we need to handle process tree killing
        command.arg("/C").arg(&config.run_command);
        if let Ok(cd) = env::current_dir() {
            command.current_dir(cd);
        }
        // Set up process group for proper cleanup
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x00000200); // CREATE_NEW_PROCESS_GROUP
        command
    }

    #[cfg(not(windows))]
    fn create_dev_command(config: CliConfig) -> Command {
        let mut command = Command::new("sh");
        command.arg("-c").arg(&config.run_command);
        if let Ok(cd) = env::current_dir() {
            command.current_dir(cd);
        }
        command
    }
}
