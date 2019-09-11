pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .map(|c| if c.is_alphabetic() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .map(abbreviate_word)
        .collect::<String>()
        .to_uppercase()
}

fn abbreviate_word(word: &str) -> String {
    let uppercase = word
        .chars()
        .filter(|c| c.is_uppercase())
        .collect::<String>();
    if uppercase.is_empty() || uppercase.len() == word.len() {
        return word.chars().next().unwrap().to_string();
    }
    uppercase
}
