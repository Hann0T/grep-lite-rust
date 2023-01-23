use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Simple grep like software
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.file).unwrap();
    let pattern = Regex::new(args.pattern.as_str()).unwrap();

    let reader = BufReader::new(file);

    for _line in reader.lines() {
        let line = _line.unwrap().to_lowercase();
        if pattern.is_match(line.as_str()) {
            println!("{:?}", line);
        }
    }
}
