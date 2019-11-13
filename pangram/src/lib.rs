use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    if sentence.chars().any(|c| !c.is_ascii()) {
        is_pangram_german(sentence)
    } else {
        sentence
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<HashSet<char>>()
            .len()
            == 26
    }
}

fn is_pangram_german(sentence: &str) -> bool {
    let mut german_alphabet = (b'a'..=b'z').map(char::from).collect::<HashSet<char>>();
    german_alphabet.extend(&['ä', 'ö', 'ü', 'ß']);
    german_alphabet
        == sentence
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<HashSet<char>>()
}
