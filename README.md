# Dev Server Monitor

A robust development server monitor that automatically restarts your server when errors are detected.

## Features

- 🔍 Monitors server output for error patterns
- 🔄 Automatic server restart on error detection
- 📝 Configurable error pattern detection
- 🧪 Test mode for functionality verification
- 📊 Process status monitoring
- 🚦 Separate stdout and stderr monitoring

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Usage

### Basic Usage
```bash
# Start monitoring your dev server
dev

# Run in test mode
dev --test
```

### Environment Setup

Add the executable to your PATH:
```bash
# Windows (PowerShell)
$env:Path += ";path\to\nextdev\target\release"
```

## Configuration

The monitor watches for `[Error` patterns by default. Key constants:
- Restart Delay: 2 seconds
- Error Recovery Delay: 5 seconds
- Process Check Interval: 100ms

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