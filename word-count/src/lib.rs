use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let words = words
        .to_lowercase()
        .split(|c: char| c.is_whitespace() || c == ',')
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    let mut result: HashMap<String, u32> = HashMap::new();

    for word in words {
        result.entry(word).and_modify(|v| *v += 1).or_insert(1);
    }

    result
}
