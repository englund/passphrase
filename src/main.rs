use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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
    file: Option<String>,
}

fn main() -> io::Result<()>  {
    let args = Cli::parse();

    if atty::is(atty::Stream::Stdin) && args.file.is_none() {
        eprintln!("No input provided. Please provide a file or pipe input.");
        std::process::exit(1);
    }

    let all_words = read_words(args.file)?;

    for _ in 0..args.num_passphrases {
        let passphrase = generate_passphrase(&all_words, args.length);
        println!("{}", passphrase);
    }

    Ok(())
}
