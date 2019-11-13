use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}
