use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut existing = HashSet::new();
    for c in candidate
        .to_lowercase()
        .chars()
        .filter(|a| a.is_alphabetic())
    {
        if existing.contains(&c) {
            return false;
        } else {
            existing.insert(c);
        }
    }
    true
}
