use clap::Parser;
use std::{error::Error, path::PathBuf};

use passphrase::{generate_passphrase, read_words};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number of words in the passphrase
    #[arg(short, long, default_value_t = 6)]
    length: u32,

    /// Number of passphrases to generate
    #[arg(short, long, default_value_t = 1)]
    num_passphrases: u32,

    /// File containing words to use for passphrases.
    /// If not provided, words are read from stdin.
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if atty::is(atty::Stream::Stdin) && args.file.is_none() {
        return Err("No input provided. Please provide a file or pipe input.".into());
    }

    let all_words = match read_words(args.file) {
        Ok(words) => words, 
        Err(e) => {
            return Err(format!("Failed to read words: {}", e).into());
        }
    };
    
    for _ in 0..args.num_passphrases {
        let passphrase = generate_passphrase(&all_words, args.length);
        println!("{}", passphrase);
    }

    Ok(())
}
