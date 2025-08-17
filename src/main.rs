use std::env;

mod config;
mod error;
mod command;
mod process;
mod monitor;
mod server;

use config::Config;
use server::DevServer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let test_mode = args.get(1).map_or(false, |arg| arg == "--test");
    
    let config = Config::new();
    let server = DevServer::new(config, test_mode);
    
    if let Err(e) = server.run() {
        eprintln!("❌ Server error: {}", e);
        std::process::exit(1);
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder: integration tests can be added here
        assert!(true);
    }
}