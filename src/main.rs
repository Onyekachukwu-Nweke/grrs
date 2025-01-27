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
    let file = File::open(&args.path).expect("could not read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(&args.pattern) {
                println!("{}", line);
            }
        }
    }

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
