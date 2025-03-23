# Rusty XML Validator

A Rust-based XML validator with WordPress WXR files in mind. This isn't quite working at the moment.

**I am not a Rust developer so I worked on this using LLMs such as ChatGPT and Claude.**

## Overview

This utility validates XML files, specifically designed for WordPress eXtended RSS (WXR) export files. It checks for invalid control characters that might cause issues during WordPress import operations.

## Features

- Validates XML files from local filesystem or URLs
- Detects problematic control characters in XML content and CDATA sections
- Supports both HTTP and HTTPS URLs
- Ignores standard whitespace characters (newlines, tabs, etc.)

## Installation

### Prerequisites
- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Building from source
```bash
git clone [repository-url]
cd rusty-xml-validator
cargo build --release
```

### Running the Validator

There are several ways to run the validator:

1. **Using cargo run** (during development):
```bash
# For local files
cargo run -- path/to/your/export.xml

# For URLs
cargo run -- https://example.com/wordpress-export.xml
```

2. **Using the compiled binary** (after building):
```bash
# The binary will be in target/release
./target/release/wxr_validator path/to/your/export.xml
```

3. **Installing globally** (optional):
```bash
cargo install --path .
wxr_validator path/to/your/export.xml
```

The validator will process the file and:
- Print any control characters it finds
- Display any XML parsing errors encountered
- Exit with status code 0 if successful, non-zero if errors occurred

### Example Output

```bash
# Successful validation (no output means no issues found)
$ wxr_validator clean-export.xml

# File with control characters
$ wxr_validator problematic-export.xml
Control character found: Some text with invalid character
Control character found: Another problematic section

# Invalid URL
$ wxr_validator https://nonexistent.com/export.xml
HTTP error: 404 Not Found
```

## Usage

```bash
# Validate a local XML file
wxr_validator path/to/file.xml

# Validate an XML file from URL
wxr_validator https://example.com/export.xml
```

## Error Messages

- "Control character found: [text]" - Indicates invalid control characters were found in the specified text
- "HTTP error: [status]" - Indicates a problem fetching the XML from a URL
- "Error at position [X]: [details]" - Indicates an XML parsing error at the specified position

## Technical Details

Built using:
- `quick-xml` for XML parsing
- `reqwest` for HTTP requests
- `clap` for command-line argument parsing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your chosen license here]
