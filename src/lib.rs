use std::{collections::HashSet, fs::File, io::{self, BufRead, BufReader}, path::PathBuf};
use rand::seq::IteratorRandom;

pub fn read_words(file: Option<PathBuf>) -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let mut all_words = Vec::new();

    let reader: Box<dyn BufRead> = match file {
        Some(file) => Box::new(BufReader::new(File::open(file)?)),
        None => Box::new(BufReader::new(stdin.lock())),
    };

    for line in reader.lines() {
        all_words.push(line?);
    }

    Ok(all_words)
}

pub fn generate_passphrase(words: &Vec<String>, num_words: u32) -> String {
    let random_words = get_unique_random_words(words, num_words);
    random_words.join(" ")
}

fn get_unique_random_words(words: &Vec<String>, num_words: u32) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut random_words = HashSet::new();

    while random_words.len() < num_words as usize {
        let random_word = match words.iter().choose(&mut rng) {
            Some(word) => word.clone(),
            None => continue,
        };
        random_words.insert(random_word);
    }

    random_words.into_iter().collect()
}
