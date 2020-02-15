use std::collections::HashMap;

const POINTS: &[(char, u64)] = &[
    ('a', 1),
    ('b', 3),
    ('c', 3),
    ('d', 2),
    ('e', 1),
    ('f', 4),
    ('g', 2),
    ('h', 4),
    ('i', 1),
    ('j', 8),
    ('k', 5),
    ('l', 1),
    ('m', 3),
    ('n', 1),
    ('o', 1),
    ('p', 3),
    ('q', 10),
    ('r', 1),
    ('s', 1),
    ('t', 1),
    ('u', 1),
    ('v', 4),
    ('w', 4),
    ('x', 8),
    ('y', 4),
    ('z', 10),
];

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let points_hash: HashMap<char, u64> = POINTS.iter().cloned().collect();
    word.to_lowercase()
        .chars()
        .filter_map(|c| points_hash.get(&c))
        .sum()
}
