use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let word = word.to_lowercase();
    let word_vec = to_vec_char(&word);

    for possible in possible_anagrams {
        let possible_lower = possible.to_lowercase();
        let possible_anagram_vec = to_vec_char(&possible_lower);
        if word_vec == possible_anagram_vec && word != possible_lower {
            result.insert(possible);
        }
    }
    result
}

fn to_vec_char(word: &String) -> Vec<char> {
    let mut vec: Vec<char> = word.chars().collect();
    vec.sort();
    vec
}