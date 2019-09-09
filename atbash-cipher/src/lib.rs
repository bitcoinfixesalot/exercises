const A_NUMBER: u8 = 97;
const Z_NUMBER: u8 = 122;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_alphanumeric() && c.is_ascii())
        .map(to_atbash_char)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(to_atbash_char)
        .collect()
}

fn to_atbash_char(symbol: char) -> char {
    if symbol.is_numeric() {
        symbol
    } else {
        (Z_NUMBER + A_NUMBER - symbol as u8) as char
    }
}
