# GitHub Copilot Instructions for Dev Server Monitor

## Project Overview

This is a **Rust-based development server monitor** that automatically detects errors in server output and restarts development servers. It's designed to be portable, configurable, and cross-platform.

## Key Functionality

- **Error Pattern Monitoring**: Watches stdout/stderr for configurable error patterns (e.g., `[Error`, `ERROR:`)
- **Automatic Server Restart**: Cleanly terminates and restarts dev servers when errors are detected
- **Portable Configuration**: Each project has its own `dev-cli.json` configuration file
- **Cross-Platform Process Management**: Handles process trees properly on Windows (`taskkill /F /T`) and Unix systems (`kill -9`)
- **Interactive Setup**: First-run configuration prompts for run command and error pattern

## Architecture Overview

The project follows a **modular architecture** with clear separation of concerns:

### Module Responsibilities

- **`main.rs`** - CLI argument parsing, Ctrl+C handling, and application entry point
- **`cli_config.rs`** - Manages portable JSON configuration (`dev-cli.json`) with interactive setup
- **`config.rs`** - Runtime configuration structure and builder patterns 
- **`error.rs`** - Custom error types (`ServerError`) and error handling
- **`command.rs`** - Cross-platform command builder with OS-specific logic
- **`process.rs`** - Process lifecycle management and proper tree termination
- **`monitor.rs`** - Output monitoring, pattern detection, and channel-based communication
- **`server.rs`** - Main orchestration logic tying all components together

### Key Design Patterns

1. **Builder Pattern**: Used in `Config` for flexible configuration construction
2. **Result Pattern**: Consistent error handling with custom `Result<T>` type
3. **Channel Communication**: Uses `std::sync::mpsc` for process monitoring
4. **Cross-Platform Abstractions**: Conditional compilation with `#[cfg(windows)]` and `#[cfg(not(windows))]`
5. **Shared State**: `Arc<Mutex<>>` for sharing child process PID across threads

## Development Guidelines

### Code Style
- Follow standard Rust conventions and formatting (`cargo fmt`)
- Use meaningful variable names and module organization
- Prefer `Result<T>` return types for error handling
- Use structured logging with emoji prefixes for user-friendly output

### Error Handling
- Custom `ServerError` enum covers all error scenarios
- Propagate errors using `?` operator consistently  
- Provide user-friendly error messages with context
- Handle both I/O errors and process management errors

### Cross-Platform Considerations
- Always use conditional compilation for OS-specific code
- Windows: Use `taskkill /F /T /PID` for process tree termination
- Unix: Use `kill -9` for process termination
- Test command parsing works across different shells
- Handle path separators and environment variables appropriately

### Testing Strategy
- Unit tests for individual modules (currently 8 tests passing)
- Integration tests for CLI configuration loading/saving
- Test mode (`--test`) for simulating errors without actual servers
- Cross-platform testing should be considered for process management

## Configuration Structure

### CLI Configuration (`dev-cli.json`)
```json
{
  "run_command": "npm run dev",
  "error_pattern": "[Error"
}
```

### Runtime Configuration (`Config`)
```rust
pub struct Config {
    pub error_pattern: String,
    pub restart_delay: Duration,     // Default: 2 seconds
    pub error_delay: Duration,       // Default: 1 second  
}
```

## Build and Development

### Building
```bash
cargo build --release    # Production build
cargo build              # Development build  
```

### Testing
```bash
cargo test               # Run all tests
```

### Running
```bash
./target/release/dev          # Start monitoring
./target/release/dev --test   # Test mode
./target/release/dev --config # Reconfigure
./target/release/dev --help   # Show help
```

## Common Patterns and Conventions

### Error Pattern Examples
- Next.js: `[Error`
- Vite: `ERROR`  
- Node.js: `Error:`
- Python Django: `Error`

### Command Examples
- `npm run dev`
- `pnpm dev`
- `yarn dev`
- `python manage.py runserver`
- `node server.js`

## Important Implementation Details

### Process Management
- Store child process PID in shared `Arc<Mutex<Option<u32>>>`
- Update PID when spawning new processes
- Use OS-appropriate termination methods
- Handle Ctrl+C gracefully with proper cleanup

### Monitor Communication
- Use channels (`std::sync::mpsc`) for error detection
- Non-blocking reads to avoid hanging
- Proper thread management for output monitoring
- Handle both stdout and stderr streams

### Configuration Management  
- Auto-create config on first run with user prompts
- Validate configuration before starting monitoring
- Support reconfiguration via `--config` flag
- Handle missing or corrupted config files gracefully

## Future Enhancements (from TODO.md)
1. LSP integration for better development experience
2. Color integration with optional settings/global settings
3. Linux testing and validation  
4. build-cli for running multiple consecutive commands

## Dependencies
- `serde` with `derive` feature for JSON serialization
- `serde_json` for configuration file handling
- `ctrlc` for graceful shutdown handling

This project prioritizes **reliability**, **portability**, and **user experience** while maintaining clean, maintainable Rust code.