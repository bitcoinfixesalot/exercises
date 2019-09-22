use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let word_vec = to_vec_char(word);
    for possible in possible_anagrams {
        let possible_anagram_vec = to_vec_char(possible);
        if word_vec == possible_anagram_vec && !is_same(word, possible) {
            result.insert(possible);
        }
    }
    result
}

fn to_vec_char(word: &str) -> Vec<char> {
    let mut vec: Vec<char> = word.to_lowercase().chars().collect();
    vec.sort();
    vec
}

fn is_same(a: &str, b: &str) -> bool {
    a.to_lowercase() == b.to_lowercase()
}
