use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
