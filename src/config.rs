use std::time::Duration;

/// Configuration constants for the dev server monitor
#[derive(Debug, Clone)]
pub struct Config {
    pub restart_delay: Duration,
    pub error_delay: Duration,
    pub process_check_interval: Duration,
    pub shutdown_timeout: Duration,
    pub error_pattern: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            restart_delay: Duration::from_secs(2),
            error_delay: Duration::from_secs(5),
            process_check_interval: Duration::from_millis(100),
            shutdown_timeout: Duration::from_secs(5), // Increased from 2 to 5 seconds
            error_pattern: "[Error".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_error_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.error_pattern = pattern.into();
        self
    }

    pub fn with_restart_delay(mut self, delay: Duration) -> Self {
        self.restart_delay = delay;
        self
    }

    pub fn with_error_delay(mut self, delay: Duration) -> Self {
        self.error_delay = delay;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.error_pattern, "[Error");
        assert_eq!(config.restart_delay, Duration::from_secs(2));
        assert_eq!(config.error_delay, Duration::from_secs(5));
    }

    #[test]
    fn test_config_builder() {
        let config = Config::new()
            .with_error_pattern("ERROR")
            .with_restart_delay(Duration::from_secs(1))
            .with_error_delay(Duration::from_secs(3));
        
        assert_eq!(config.error_pattern, "ERROR");
        assert_eq!(config.restart_delay, Duration::from_secs(1));
        assert_eq!(config.error_delay, Duration::from_secs(3));
    }

    #[test]
    fn test_config_new() {
        let config = Config::new();
        // Should be equivalent to default
        let default_config = Config::default();
        assert_eq!(config.error_pattern, default_config.error_pattern);
        assert_eq!(config.restart_delay, default_config.restart_delay);
    }
}
