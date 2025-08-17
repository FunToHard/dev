use std::fmt;

/// Custom error types for the dev server monitor
#[derive(Debug)]
pub enum ServerError {
    ProcessStart(String),
    IoError(String),
    ChannelError(String),
    ProcessManagement(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::ProcessStart(msg) => write!(f, "Failed to start process: {}", msg),
            ServerError::IoError(msg) => write!(f, "IO error: {}", msg),
            ServerError::ChannelError(msg) => write!(f, "Channel communication error: {}", msg),
            ServerError::ProcessManagement(msg) => write!(f, "Process management error: {}", msg),
        }
    }
}

impl std::error::Error for ServerError {}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IoError(err.to_string())
    }
}

impl From<std::sync::mpsc::SendError<crate::monitor::WatchMessage>> for ServerError {
    fn from(err: std::sync::mpsc::SendError<crate::monitor::WatchMessage>) -> Self {
        ServerError::ChannelError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ServerError>;
