use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    code(key, s, shift_right)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    code(key, s, shift_left)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key: String = (0..100)
        .map(|_| rng.gen_range(b'a', b'z' + 1) as char)
        .collect();
    let cipher = encode(&key, s).unwrap();
    (key, cipher)
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

fn code<F>(key: &str, s: &str, shift_fn: F) -> Option<String>
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
