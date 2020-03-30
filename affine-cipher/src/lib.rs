use std::ops::{Rem, Sub};

const MODULUS: i32 = 26;
/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, MODULUS) != 1 { return Err(AffineCipherError::NotCoprime(a)); }
    let encode_char = |c: char| {
        if c.is_ascii_digit() {
            c
        } else {
            let idx = (c as i32) - ('a' as i32);
            let encode = (idx * a + b) % MODULUS + 'a' as i32;
            encode as u8 as char
        }
    };
    let ret = plaintext.chars()
                       .filter(char::is_ascii_alphanumeric)
                       .map(|c| c.to_ascii_lowercase())
                       .map(|c| encode_char(c))
                       .collect::<Vec::<char>>()
                       .chunks(5)
                       .map(|slice| slice.iter().cloned().collect::<String>())
                       .collect::<Vec<String>>()
                       .join(" ");
    Ok(ret)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, MODULUS) != 1 { return Err(AffineCipherError::NotCoprime(a)); }
    let inv: i32 = (1..).filter(|i| (*i * a).rem_euclid(MODULUS) == 1)
                        .next()
                        .unwrap();
    let decode_char = |c: char| {
        if c.is_ascii_digit() {
            c
        } else {
            let idx = (c as i32) - ('a' as i32);
            let decode = (inv * (idx - b)).rem_euclid(MODULUS) + 'a' as i32;
            decode as u8 as char
        }
    };
    let ret: String = ciphertext.chars()
                                .filter(char::is_ascii_alphanumeric)
                                .map(decode_char)
                                .collect();
    Ok(ret)
}
fn gcd<T>(a: T, b: T) -> T
where T: Eq + PartialOrd + Sub<Output=T> + Rem<Output=T> + Copy,
{
    if a % b == a - a {
        b
    } else {
        gcd(b, a % b)
    }
}
