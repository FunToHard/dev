use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::error::{Result, ServerError};

const CONFIG_FILE: &str = "dev-cli.json";

/// CLI configuration that gets saved to dev-cli.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub run_command: String,
    pub error_pattern: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            run_command: "pnpm dev".to_string(),
            error_pattern: "[Error".to_string(),
        }
    }
}

impl CliConfig {
    /// Load configuration from dev-cli.json or create it if it doesn't exist
    pub fn load_or_create() -> Result<Self> {
        let config_path = Path::new(CONFIG_FILE);

        if config_path.exists() {
            println!("📄 Loading configuration from {}", CONFIG_FILE);
            Self::load_from_file(config_path)
        } else {
            println!("📄 Configuration file {} not found", CONFIG_FILE);
            Self::create_interactive()
        }
    }

    /// Load configuration from existing file
    fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| ServerError::IoError(format!("Failed to read {}: {}", CONFIG_FILE, e)))?;

        let config: CliConfig = serde_json::from_str(&content)
            .map_err(|e| ServerError::IoError(format!("Failed to parse {}: {}", CONFIG_FILE, e)))?;

        println!("✅ Loaded configuration:");
        println!("   Run command: {}", config.run_command);
        println!("   Error pattern: {}", config.error_pattern);

        Ok(config)
    }

    /// Create configuration interactively and save to file
    fn create_interactive() -> Result<Self> {
        println!("🔧 Let's set up your dev server configuration!");
        println!();

        // Get run command
        print!("Enter the command to run your dev server [default: pnpm dev]: ");
        io::stdout().flush().unwrap();
        let mut run_command = String::new();
        io::stdin()
            .read_line(&mut run_command)
            .map_err(|e| ServerError::IoError(format!("Failed to read input: {}", e)))?;
        let run_command = run_command.trim();
        let run_command = if run_command.is_empty() {
            "pnpm dev".to_string()
        } else {
            run_command.to_string()
        };

        // Get error pattern
        print!("Enter the error pattern to watch for [default: [Error]: ");
        io::stdout().flush().unwrap();
        let mut error_pattern = String::new();
        io::stdin()
            .read_line(&mut error_pattern)
            .map_err(|e| ServerError::IoError(format!("Failed to read input: {}", e)))?;
        let error_pattern = error_pattern.trim();
        let error_pattern = if error_pattern.is_empty() {
            "[Error".to_string()
        } else {
            error_pattern.to_string()
        };

        let config = CliConfig {
            run_command,
            error_pattern,
        };

        // Save to file
        config.save_to_file()?;

        println!();
        println!("✅ Configuration saved to {}", CONFIG_FILE);
        println!("   Run command: {}", config.run_command);
        println!("   Error pattern: {}", config.error_pattern);
        println!();

        Ok(config)
    }

    /// Save configuration to dev-cli.json
    fn save_to_file(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ServerError::IoError(format!("Failed to serialize config: {}", e)))?;

        fs::write(CONFIG_FILE, json)
            .map_err(|e| ServerError::IoError(format!("Failed to write {}: {}", CONFIG_FILE, e)))?;

        Ok(())
    }

    /// Get the command parts for execution
    pub fn get_command_parts(&self) -> Vec<&str> {
        self.run_command.split_whitespace().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_default_config() {
        let config = CliConfig::default();
        assert_eq!(config.run_command, "pnpm dev");
        assert_eq!(config.error_pattern, "[Error");
    }

    #[test]
    fn test_get_command_parts() {
        let config = CliConfig {
            run_command: "npm run dev".to_string(),
            error_pattern: "[Error".to_string(),
        };
        let parts = config.get_command_parts();
        assert_eq!(parts, vec!["npm", "run", "dev"]);
    }

    #[test]
    fn test_json_serialization() {
        let config = CliConfig {
            run_command: "yarn dev".to_string(),
            error_pattern: "ERROR:".to_string(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CliConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.run_command, deserialized.run_command);
        assert_eq!(config.error_pattern, deserialized.error_pattern);
    }

    #[test]
    fn test_save_and_load_config() {
        let test_file = "test-cli-config.json";

        // Clean up any existing test file
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).unwrap();
        }

        let original_config = CliConfig {
            run_command: "bun dev".to_string(),
            error_pattern: "Error:".to_string(),
        };

        // Save config
        let json = serde_json::to_string_pretty(&original_config).unwrap();
        fs::write(test_file, json).unwrap();

        // Load config
        let loaded_config = CliConfig::load_from_file(Path::new(test_file)).unwrap();

        assert_eq!(original_config.run_command, loaded_config.run_command);
        assert_eq!(original_config.error_pattern, loaded_config.error_pattern);

        // Clean up
        fs::remove_file(test_file).unwrap();
    }
}
