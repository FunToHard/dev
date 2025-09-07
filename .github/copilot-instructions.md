# Dev Server Monitor - GitHub Copilot Instructions

**ALWAYS follow these instructions first.** Only fallback to additional search and context gathering if the information here is incomplete or found to be in error.

## Project Overview

Dev Server Monitor is a Rust CLI application that monitors development server output for error patterns and automatically restarts servers when errors are detected. It creates portable, per-project JSON configuration files and supports cross-platform development.

## Working Effectively

### Environment Setup
- Rust is pre-installed (rustc 1.89.0, cargo 1.89.0)
- No additional SDK installations required
- No external dependencies or databases needed

### Build and Test Commands
Run these commands in sequence to build and validate the project:

```bash
# Build debug version (takes ~15 seconds initially, faster on subsequent builds)
cargo build

# Build release version (takes ~8 seconds, recommended for final validation)
cargo build --release

# Run tests (takes <1 second, 8 tests)
cargo test

# Run linter (takes ~3.5 seconds, may show dead code warnings which are acceptable)
cargo clippy

# Format code (required before committing)
cargo fmt
```

**NEVER CANCEL these builds.** While initial builds may take 15+ seconds due to dependency downloads, subsequent builds are much faster.

### Validation Scenarios
ALWAYS validate your changes using the test mode after making modifications:

```bash
# Test the core functionality (error detection and restart logic)
./target/release/dev --test
# This simulates a server with error patterns - you should see automatic restarts
# Press Ctrl+C to stop the test

# Test help functionality
./target/release/dev --help

# Test configuration creation
./target/release/dev --config
```

### Before Committing
ALWAYS run these commands before committing or the build will fail:
```bash
cargo fmt        # Format code (required)
cargo clippy     # Check for issues
cargo test       # Ensure tests pass
```

## Project Structure and Navigation

### Key Files and Directories
```
/home/runner/work/dev/dev/
├── src/                    # Source code (8 modules)
│   ├── main.rs            # CLI entry point and argument parsing
│   ├── cli_config.rs      # Portable JSON configuration management
│   ├── config.rs          # Runtime configuration and constants
│   ├── error.rs           # Custom error types and error handling
│   ├── command.rs         # Cross-platform command builder
│   ├── process.rs         # Process lifecycle management
│   ├── monitor.rs         # Output monitoring and pattern detection
│   └── server.rs          # Main orchestration logic
├── Cargo.toml             # Rust project configuration
├── Cargo.lock             # Dependency lock file
├── README.md              # Project documentation
├── TODO.md                # Future enhancements
└── target/                # Build outputs (gitignored)
    ├── debug/dev          # Debug executable
    └── release/dev        # Release executable (preferred)
```

### Frequently Modified Areas
When making changes, you'll commonly work with:
- **`main.rs`** - CLI argument handling, help text, Ctrl+C handling
- **`server.rs`** - Main application logic and orchestration
- **`cli_config.rs`** - Configuration file handling and user prompts
- **`process.rs`** - Process management and termination logic
- **`monitor.rs`** - Error pattern detection and monitoring logic

## Common Development Tasks

### Building and Running
```bash
# Debug build and run with help
cargo build && ./target/debug/dev --help

# Release build and run (preferred for testing)
cargo build --release && ./target/release/dev --help

# Test functionality
cargo build --release && ./target/release/dev --test
```

### Testing Changes
1. Make your changes to the source code
2. Run `cargo build --release` (8 seconds)
3. Test using `./target/release/dev --test` to validate error detection works
4. Test help with `./target/release/dev --help`
5. Run `cargo test` to ensure unit tests pass
6. Run `cargo clippy` and `cargo fmt` before committing

### Adding New Features
- Add unit tests in the same file using `#[cfg(test)]` modules
- Follow the existing modular architecture pattern
- Update help text in `main.rs` if adding new CLI options
- Test cross-platform compatibility using conditional compilation (`#[cfg(windows)]`, `#[cfg(not(windows))]`)

## Architecture Notes

### Modular Design
The application uses a clean modular architecture:
- **Configuration Layer**: `cli_config.rs` handles portable JSON config files
- **Process Layer**: `process.rs` manages child process lifecycle with proper tree termination
- **Monitoring Layer**: `monitor.rs` watches output streams for error patterns
- **Command Layer**: `command.rs` builds cross-platform command structures
- **Orchestration**: `server.rs` coordinates all components

### Key Dependencies
- `serde` and `serde_json` - JSON configuration serialization
- `ctrlc` - Ctrl+C signal handling for clean shutdown

### Cross-Platform Considerations
- Uses `taskkill /F /T` on Windows for process tree termination
- Uses `kill -9` on Unix-like systems
- Conditional compilation directives handle platform differences

## Expected Timing and Performance
- **Debug build**: 15 seconds initially, <5 seconds subsequently
- **Release build**: 8 seconds initially, <3 seconds subsequently  
- **Tests**: <1 second (8 tests total)
- **Linting (clippy)**: ~3.5 seconds
- **Code formatting**: <1 second

## Troubleshooting
- **Build fails**: Run `cargo clean` then `cargo build`
- **Tests fail**: Check that you haven't broken the modular architecture
- **Clippy warnings**: Dead code warnings are acceptable, fix other issues
- **Format issues**: Always run `cargo fmt` before committing

## Application Usage Examples
```bash
# Start monitoring in a project directory (creates config if needed)
dev

# Test the error detection system
dev --test

# Reconfigure settings
dev --config

# Show help
dev --help
```

The application creates a `dev-cli.json` file in each project directory with settings like:
```json
{
  "run_command": "npm run dev",
  "error_pattern": "[Error"
}
```