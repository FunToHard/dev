# Dev Server Monitor

A robust development server monitor that automatically restarts processes when errors are detected in their output.

## Architecture

The application has been refactored into a modular, maintainable structure:

### Modules

- **`main.rs`** - Entry point and CLI argument parsing
- **`config.rs`** - Configuration management and constants
- **`error.rs`** - Custom error types and error handling
- **`command.rs`** - Command builder for different execution modes
- **`process.rs`** - Process lifecycle management
- **`monitor.rs`** - Output monitoring and pattern detection
- **`server.rs`** - Main server orchestration logic

### Key Features

- **Modular Design**: Each module has a single responsibility
- **Error Handling**: Comprehensive error types with proper propagation
- **Configuration**: Centralized configuration with builder pattern
- **Cross-Platform**: Supports both Windows and Unix-like systems
- **Test Mode**: Built-in test mode for validation

## Usage

### Normal Mode (monitors `pnpm dev`)
```bash
cargo run
```

### Test Mode (simulates error scenarios)
```bash
cargo run -- --test
```

## Configuration

The `Config` struct provides several customization options:

```rust
let config = Config::new()
    .with_error_pattern("[Error")
    .with_restart_delay(Duration::from_secs(2))
    .with_error_delay(Duration::from_secs(5));
```

## Error Detection

The monitor watches both stdout and stderr for the configured error pattern (default: `[Error`). When detected:

1. The current process is gracefully terminated
2. A restart delay is applied
3. The process is restarted with a new attempt counter

## Benefits of the Refactored Architecture

1. **Maintainability**: Code is organized into logical modules
2. **Testability**: Each module can be tested independently
3. **Extensibility**: New features can be added without affecting existing code
4. **Reusability**: Components can be reused in other projects
5. **Error Handling**: Proper error propagation and handling throughout
6. **Configuration**: Easy to modify behavior without code changes

## Future Enhancements

The modular structure makes it easy to add:

- Configuration file support
- Multiple error patterns
- Custom restart strategies
- Logging integration
- Performance metrics
- Health checks

## Requirements

- Windows environment
- pnpm installed globally (for dev server)
- Rust 2021 edition or later

## Error Handling

The monitor handles various error scenarios:
- Process startup failures
- I/O errors during monitoring
- Channel communication errors
- Process termination errors

## Development

### Running Tests
```bash
cargo test
```

### Building in Debug Mode
```bash
cargo build
```

### Building for Release
```bash
cargo build --release
```

## License

MIT License

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## Support

For issues and feature requests, please open an issue on GitHub.