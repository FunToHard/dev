# Release Workflows

This repository includes GitHub Actions workflows for automated building and releasing of the Dev Server Monitor CLI tool.

## Workflows

### Release Workflow (`.github/workflows/release.yml`)

This workflow automatically builds and releases binaries for multiple platforms when you push a version tag.

**Supported Platforms:**
- Linux x86_64
- Windows x86_64  
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)

**How to Create a Release:**

1. Update the version in `Cargo.toml` if needed
2. Create and push a git tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. The workflow will automatically:
   - Build binaries for all platforms
   - Create a GitHub release
   - Attach the binaries as release assets

**Binary Names:**
- `dev-linux-x86_64` - Linux binary
- `dev-windows-x86_64.exe` - Windows binary
- `dev-macos-x86_64` - macOS Intel binary  
- `dev-macos-aarch64` - macOS Apple Silicon binary

### CI Workflow (`.github/workflows/ci.yml`)

This workflow runs on every push to main/master and on pull requests to ensure code quality:

- **Tests**: Runs `cargo test`
- **Build**: Runs `cargo build --release`
- **Formatting**: Checks code formatting with `cargo fmt --check`
- **Linting**: Runs `cargo clippy` with appropriate warning levels

## Usage Example

After creating a release, users can download the appropriate binary for their platform:

```bash
# Linux/macOS
wget https://github.com/FunToHard/dev/releases/download/v0.1.0/dev-linux-x86_64
chmod +x dev-linux-x86_64
./dev-linux-x86_64

# Or install to PATH
sudo mv dev-linux-x86_64 /usr/local/bin/dev
dev --help
```

## Development

The workflows are designed to work out of the box. No additional configuration is needed - just push a tag to create a release!