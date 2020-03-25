use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
pub fn encode(key: &str, s: &str) -> Option<String> {
    shift_all(key, s, shift_right)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    shift_all(key, s, shift_left)
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_lowercase())
        .take(100)
        .collect();
    let encoded = encode(&key, s);
    (key, encoded.unwrap())
}

fn shift(symbol: char, by: i8) -> char {
    let symbol = symbol as u8 - b'a';
    (((symbol as i32) + (by as i32)).rem_euclid(26) as u8 + b'a') as char
}

fn shift_right(symbol: char, by: char) -> char {
    shift(symbol, (by as u8 - b'a') as i8)
}

fn shift_left(symbol: char, by: char) -> char {
    shift(symbol, -((by as u8 - b'a') as i8))
}

fn shift_all<F>(key: &str, s: &str, shift_fn: F) -> Option<String>
where
    F: Fn(char, char) -> char,
{
    match key {
        "" => None,
        key => s
            .chars()
            .zip(key.chars().cycle())
            .map(|(s, k)| match (s, k) {
                ('a'..='z', 'a'..='z') => Some(shift_fn(s, k)),
                _ => None,
            })
            .collect(),
    }
}
