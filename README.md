# Dev Server Monitor

A portable, configurable development server monitor that automatically restarts processes when errors are detected in their output.

## 🚀 Features

- **🔧 Portable Configuration**: Automatically creates `dev-cli.json` in your project directory
- **📝 Interactive Setup**: Prompts for run command and error pattern on first use
- **🔄 Smart Restart Logic**: Properly terminates process trees and restarts on errors
- **🧪 Test Mode**: Built-in test mode for validation
- **📊 Cross-Platform**: Supports both Windows and Unix-like systems
- **⚡ Modular Architecture**: Clean, maintainable codebase

## 📦 Installation

1. Clone this repository
2. Build the release version:
   ```bash
   cargo build --release
   ```
3. Copy `target/release/dev.exe` to a directory in your PATH

## 🎯 Usage

### First Time Setup
When you run `dev` in a new project directory, it will automatically prompt you to configure:

```bash
dev
```

You'll be asked to provide:
- **Run Command**: The command to start your dev server (e.g., `npm run dev`, `pnpm dev`, `yarn dev`)
- **Error Pattern**: The text pattern to watch for errors (e.g., `[Error`, `ERROR:`, `Error:`)

### Common Commands

```bash
# Start monitoring (auto-configures on first run)
dev

# Test the error detection
dev --test

# Reconfigure settings
dev --config

# Show help
dev --help
```

## 📄 Configuration File

The CLI creates a `dev-cli.json` file in your project directory:

```json
{
  "run_command": "npm run dev",
  "error_pattern": "[Error"
}
```

This makes the tool completely portable - just copy the executable to any project and it will work with that project's specific configuration.

## 🏗️ Architecture

### Modular Design
- **`main.rs`** - CLI argument parsing and entry point
- **`cli_config.rs`** - Portable JSON configuration management
- **`config.rs`** - Runtime configuration and constants
- **`error.rs`** - Custom error types and error handling
- **`command.rs`** - Cross-platform command builder
- **`process.rs`** - Process lifecycle management with proper tree termination
- **`monitor.rs`** - Output monitoring and pattern detection
- **`server.rs`** - Main orchestration logic

### Key Benefits

1. **Portability**: Each project has its own configuration
2. **Flexibility**: Supports any command and error pattern
3. **Reliability**: Proper process tree termination on Windows
4. **Maintainability**: Clean modular architecture
5. **User-Friendly**: Interactive setup and clear help messages

## 🔧 How It Works

1. **Configuration Loading**: Checks for `dev-cli.json` in current directory
2. **Interactive Setup**: If no config found, prompts user for settings
3. **Process Monitoring**: Spawns the configured command and monitors output
4. **Error Detection**: Watches stdout/stderr for the configured error pattern
5. **Smart Restart**: Uses `taskkill /F /T` on Windows to terminate process trees
6. **Automatic Retry**: Restarts the server after a configurable delay

## 🎯 Example Configurations

### Next.js Project
```json
{
  "run_command": "npm run dev",
  "error_pattern": "[Error"
}
```

### Vite React Project
```json
{
  "run_command": "npm run dev",
  "error_pattern": "ERROR"
}
```

### Custom Node.js Server
```json
{
  "run_command": "node server.js",
  "error_pattern": "Error:"
}
```

### Python Django
```json
{
  "run_command": "python manage.py runserver",
  "error_pattern": "Error"
}
```

## 🚀 Benefits Over Other Solutions

1. **No Global Configuration**: Each project maintains its own settings
2. **Zero Dependencies**: Single executable, no runtime dependencies
3. **Smart Process Management**: Properly handles complex process trees
4. **Cross-Platform**: Works on Windows, macOS, and Linux
5. **Fast Setup**: Interactive configuration in seconds
6. **Reliable Termination**: Uses OS-specific methods for clean shutdowns

## 🔮 Future Enhancements

The modular architecture makes it easy to add:
- Multiple error patterns
- Custom restart strategies
- Webhooks and notifications
- Logging and metrics
- Health checks
- Configuration templates

## 📋 Requirements

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