# panic-pretty

Transform ugly panic stack traces into beautiful, readable output with syntax highlighting and smart filtering

## Features

- Read stack traces from stdin or file paths via CLI arguments
- Auto-detect trace format (Rust panic, Go panic, Node.js Error)
- Parse stack frames extracting file paths, line numbers, function names
- Apply syntax highlighting with different colors for files, functions, line numbers
- Filter out standard library and vendor frames with --filter flag
- Show configurable number of context lines from source files when available
- Support --no-color flag for CI/log file compatibility
- Display summary statistics (total frames, filtered frames, unique files)
- Handle multiple stack traces in single input
- Preserve original trace as fallback if parsing fails

## How to Use

Use this project when you need to:

- Quickly solve problems related to panic-pretty
- Integrate rust functionality into your workflow
- Learn how rust handles common patterns

## Installation

```bash
# Clone the repository
git clone https://github.com/KurtWeston/panic-pretty.git
cd panic-pretty

# Install dependencies
cargo build
```

## Usage

```bash
cargo run
```

## Built With

- rust

## Dependencies

- `clap`
- `colored`
- `regex`
- `anyhow`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
