use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

#[test]
fn test_pattern_matching() {
    let pattern = "test";
    let test_string = "this is a test string";
    assert!(test_string.contains(pattern));
}

#[test]
fn test_cli_integration() {
    // Create a temporary file for testing
    let temp_file_path = "test_file.txt";
    let mut file = File::create(temp_file_path).unwrap();
    writeln!(file, "this is a test line").unwrap();
    writeln!(file, "this line does not match").unwrap();
    writeln!(file, "another test line").unwrap();

    // Run the CLI command
    let output = Command::new("cargo")
        .args(&["run", "--", "--pattern", "test", "--path", temp_file_path])
        .output()
        .expect("Failed to execute command");

    // Check the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("this is a test line"));
    assert!(stdout.contains("another test line"));
    assert!(!stdout.contains("this line does not match"));

    // Clean up the temporary file
    fs::remove_file(temp_file_path).unwrap();
}
