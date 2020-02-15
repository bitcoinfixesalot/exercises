const X: char = 'X';

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.chars().filter_map(to_digit).collect::<Vec<u32>>();

    if isbn.len() != 10 || isbn[..9].iter().any(|d| *d == 10) {
        false
    } else {
        isbn.iter()
            .enumerate()
            .map(|(i, d)| *d * (10 - i as u32))
            .sum::<u32>()
            % 11
            == 0
    }
}

fn to_digit(c: char) -> Option<u32> {
    match c {
        _ if c.is_numeric() => Some(c.to_digit(10).unwrap()),
        X => Some(10),
        _ => None,
    }
}
