# Pattern Search CLI

A command-line tool written in Rust that searches for specific patterns within text files. The application provides detailed logging and error handling capabilities.

## Features

- Search for text patterns in files
- Detailed error messages
- Configurable logging verbosity
- Command-line argument parsing with help documentation

## Installation

1. Make sure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs)

2. Clone the repository:
```bash
git clone [your-repository-url]
cd pattern-search-cli
```

3. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/`

## Usage

Basic usage:
```bash
pattern-search-cli <PATTERN> <FILE_PATH>
```

With verbose logging:
```bash
pattern-search-cli <PATTERN> <FILE_PATH> -v
```

Example:
```bash
pattern-search-cli "Hello" sample.txt
pattern-search-cli "Error" logs/app.log -vvv
```

### Command Line Arguments

- `<PATTERN>`: The text pattern to search for in the file
- `<FILE_PATH>`: Path to the file to search in
- `-v, --verbose`: Enable verbose output (can be used multiple times for increased verbosity)
- `-h, --help`: Display help information
- `-V, --version`: Display version information

## Testing

1. Create a test file:
```bash
echo "Hello World\nTest Line\nHello Rust" > test.txt
```

2. Run basic tests:
```bash
# Should find two lines containing "Hello"
pattern-search-cli "Hello" test.txt

# Should find one line containing "Test"
pattern-search-cli "Test" test.txt

# Test with verbose logging
pattern-search-cli "Hello" test.txt -v
```

3. Test error handling:
```bash
# Test with non-existent file
pattern-search-cli "pattern" nonexistent.txt

# Test with unreadable file (create file without read permissions)
touch unreadable.txt
chmod 000 unreadable.txt
pattern-search-cli "pattern" unreadable.txt
```

## Dependencies

- `clap`: Command line argument parsing
- `anyhow`: Error handling
- `log` and `env_logger`: Logging functionality
- `std`: Rust standard library components

## Error Handling

The application handles several error cases:
- File not found
- Permission denied
- Invalid file content
- File reading errors

Error messages include:
- Detailed context about the failure
- File path information
- Underlying system error details

## Development

To contribute to the project:

1. Fork the repository
2. Create a feature branch
3. Run tests: `cargo test`
4. Submit a pull request
