use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::{BufReader, Result};

#[derive(Parser, Debug)]
#[command(author = "Mani", about = "Searches for patterns", long_about = None)]
struct Args {
    #[arg(help = "The Pattern to search for")]
    pattern: String,

    #[arg(help = "File to search")]
    input: Option<String>,
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let pattern = args.pattern;
    let input = args.input.unwrap_or(String::from("-"));
    let re = Regex::new(&pattern).unwrap();

    println!(
        "Pattern to search for: {}\nFile to search in: {:?}",
        &pattern, input
    );

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input)?;
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }

    Ok(())
}
