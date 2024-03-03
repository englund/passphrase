use std::collections::HashSet;
use rand::seq::IteratorRandom;

pub fn generate_passphrase(words: &Vec<String>, num_words: u32) -> String {
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
