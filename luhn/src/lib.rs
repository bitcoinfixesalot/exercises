use std::collections::HashSet;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let allowed_chars: HashSet<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ']
        .into_iter()
        .collect();

    if !code.chars().all(|c| allowed_chars.contains(&c)) {
        return false;
    }

    if code.chars().filter(|c| c.is_digit(10)).count() <= 1 {
        return false;
    }

    let sum = code
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .fold(0, |acc, (index, n)| {
            if index % 2 == 0 {
                acc + n
            } else if n * 2 > 9 {
                acc + n * 2 - 9
            } else {
                acc + n * 2
            }
        });

    sum % 10 == 0
}
