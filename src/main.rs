use clap::Parser;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = File::open(&args.path).unwrap_or_else(|_| {
        eprintln!("Error: Could not open file at path: {:?}", args.path);
        std::process::exit(1);
    });
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
