use clap::Parser;
use std::io::{self, BufRead};

use passphrase::generate_passphrase;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number of words in the passphrase
    #[arg(short, long, default_value_t = 6)]
    length: u32,

    /// Number of passphrases to generate
    #[arg(short, long, default_value_t = 1)]
    num_passphrases: u32,
}

fn main() {
    let args = Cli::parse();

    let stdin = io::stdin();
    let mut all_words = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        all_words.push(line);
    }

    for _ in 0..args.num_passphrases {
        let passphrase = generate_passphrase(&all_words, args.length);
        println!("{}", passphrase);
    }
}
