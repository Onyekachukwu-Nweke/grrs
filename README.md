# GRRS

A simple command-line interface (CLI) application written in Rust that allows users to search for specific patterns within text files. The application reads a file specified by the user and prints lines that contain the given pattern, with robust error handling and informative messages.

## Features

- Search for patterns in text files.
- Custom error messages for file handling and reading issues.
- Comprehensive unit and integration tests to ensure reliability.
- Easy to use with command-line arguments.

## Usage

To run the application, use the following command:

```bash
cargo run -- --pattern <PATTERN> --path <FILE_PATH>
```

Replace `<PATTERN>` with the string you want to search for and `<FILE_PATH>` with the path to the text file.

## Testing

Run the tests using:

```bash
cargo test
```

This will execute both unit and integration tests to verify the functionality of the application.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.

