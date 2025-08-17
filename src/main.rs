use std::env;

mod config;
mod error;
mod command;
mod process;
mod monitor;
mod server;
mod cli_config;

use config::Config;
use server::DevServer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let test_mode = args.iter().any(|arg| arg == "--test");
    let config_mode = args.iter().any(|arg| arg == "--config");
    let help_mode = args.iter().any(|arg| arg == "--help" || arg == "-h");
    
    if help_mode {
        print_help();
        return;
    }
    
    if config_mode {
        println!("🔧 Reconfiguring dev-cli.json...");
        if let Err(e) = create_config_interactive() {
            eprintln!("❌ Configuration error: {}", e);
            std::process::exit(1);
        }
        return;
    }
    
    let config = Config::new();
    let mut server = DevServer::new(config, test_mode);
    
    if let Err(e) = server.run() {
        eprintln!("❌ Server error: {}", e);
        std::process::exit(1);
    }
}

fn print_help() {
    println!("🚀 Dev Server Monitor - Portable Development Server Watcher");
    println!();
    println!("USAGE:");
    println!("    dev [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --test      Run in test mode (simulates errors for testing)");
    println!("    --config    Create or update dev-cli.json configuration");
    println!("    --help, -h  Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Monitors your development server output for error patterns and automatically");
    println!("    restarts the server when errors are detected. On first run in a directory,");
    println!("    you'll be prompted to configure the run command and error pattern.");
    println!();
    println!("CONFIGURATION:");
    println!("    Configuration is stored in 'dev-cli.json' in your project directory.");
    println!("    Example configuration:");
    println!("    {{");
    println!("      \"run_command\": \"npm run dev\",");
    println!("      \"error_pattern\": \"[Error\"");
    println!("    }}");
    println!();
    println!("EXAMPLES:");
    println!("    dev                    # Start monitoring (creates config if needed)");
    println!("    dev --test             # Test the error detection in test mode");
    println!("    dev --config           # Reconfigure the run command and error pattern");
}

fn create_config_interactive() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use crate::cli_config::CliConfig;
    use std::fs;
    
    // Remove existing config if it exists
    if std::path::Path::new("dev-cli.json").exists() {
        fs::remove_file("dev-cli.json")?;
        println!("📄 Removed existing dev-cli.json");
    }
    
    // Create new config
    let _config = CliConfig::load_or_create()?;
    println!("✅ Configuration complete! You can now run 'dev' to start monitoring.");
    
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder: integration tests can be added here
        assert!(true);
    }
}