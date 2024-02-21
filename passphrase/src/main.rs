
use clap::Parser;
use std::{collections::HashSet, io::{self, BufRead}};

use rand::seq::IteratorRandom;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Number of words in the passphrase
    #[arg(long, default_value_t = 6)]
    num_words: u32,

    /// Number of passphrases to generate
    #[arg(long, default_value_t = 1)]
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
        let passphrase = generate_passphrase(&all_words, args.num_words);
        println!("{}", passphrase);
    }
}

fn generate_passphrase(words: &Vec<String>, num_words: u32) -> String {
    let random_words = get_unique_random_words(words, num_words);
    let passphrase = random_words.join(" ");
    passphrase
}

fn get_unique_random_words(words: &Vec<String>, num_words: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut random_words = HashSet::new();

    while random_words.len() < num_words as usize {
        let random_word = words.iter().choose(&mut rng).unwrap().clone();
        random_words.insert(random_word);
    }

    random_words.into_iter().collect()
}
