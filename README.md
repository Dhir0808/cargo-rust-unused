# cargo-rust-unused

A CLI tool for detecting unused code in Rust projects. This tool helps you maintain clean and efficient codebases by identifying:

- Unused dependencies in `Cargo.toml`
- Unused functions and methods
- Unused modules

## Features

- 🔍 Analyzes Rust source code using `syn` for accurate parsing
- 📦 Detects unused dependencies in `Cargo.toml`
- 🚀 Fast and efficient file traversal
- 💡 Smart detection of unused code items
- 🎨 Beautiful colored output
- 📊 Optional JSON output for CI integration

## Installation

### From crates.io

```bash
cargo install cargo-rust-unused
```

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/cargo-rust-unused
cd cargo-rust-unused

# Build and install locally
cargo install --path .
```

## Usage

The tool can be used in two ways:

### 1. As a Cargo Subcommand

```bash
cargo rust-unused [OPTIONS]
```

### 2. As a Standalone Binary

```bash
cargo-rust-unused [OPTIONS]
```

### Options

- `--path <PATH>`: Path to the Rust project (default: current directory)
- `--format <FORMAT>`: Output format [text|json] (default: text)
- `--include-private`: Include private items in the analysis
- `--help`: Print help information
- `--version`: Print version information

### Example Usage

1. **Analyze Current Project**

```bash
# Navigate to your Rust project
cd my-rust-project

# Run the analysis
cargo rust-unused
```

2. **Analyze with JSON Output**

```bash
cargo rust-unused --format json
```

3. **Analyze Specific Project Path**

```bash
cargo rust-unused --path ~/projects/my-rust-project
```

4. **Save Analysis Results**

```bash
# Save as text
cargo rust-unused > analysis_report.txt

# Save as JSON
cargo rust-unused --format json > analysis_report.json
```

### Example Output

```bash
🔍 Analyzing project...

Unused Dependencies:
  - regex
  - serde_yaml

Unused Functions:
  - fn unused_utility()
  - fn deprecated_function()

Unused Modules:
  - mod old_features
```

### JSON Output Format

```json
{
  "unused_dependencies": ["regex", "serde_yaml"],
  "unused_functions": ["fn unused_utility", "fn deprecated_function"],
  "unused_modules": ["mod old_features"]
}
```

## Integration with CI/CD

You can integrate the tool into your CI/CD pipeline. Here's an example GitHub Actions workflow:

```yaml
name: Code Analysis

on: [push, pull_request]

jobs:
  unused-code-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Install cargo-rust-unused
        run: cargo install --path .

      - name: Run Analysis
        run: |
          cargo rust-unused --format json > analysis.json

      - name: Check Results
        run: |
          if [ "$(jq '.unused_dependencies | length' analysis.json)" -gt 0 ]; then
            echo "Warning: Found unused dependencies!"
            exit 1
          fi
```

## Best Practices

1. **Regular Usage**:

   - Run before committing changes
   - Include in pre-commit hooks
   - Run as part of code review process

2. **Handling Results**:

   - Review unused dependencies first (they increase build time)
   - Check unused functions for potential dead code
   - Verify unused modules aren't needed for future use

3. **False Positives**:
   - Some items might be used indirectly (through macros or reflection)
   - Dependencies might be needed for dev/test environments
   - Consider using `#[allow(dead_code)]` for intentionally unused items

## Troubleshooting

### Common Issues

1. **Tool Not Finding Files**:

   - Ensure you're in a Rust project directory
   - Check if `Cargo.toml` exists
   - Verify `src` directory exists

2. **Incorrect Results**:
   - Update to the latest version
   - Try with `--include-private` flag
   - Check if code uses macros or dynamic dispatch

### Error Messages

```
Error: Not a Cargo project: no Cargo.toml found
Solution: Run from a Rust project directory

Error: No src directory found
Solution: Create src directory or check path
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Here's how you can contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
