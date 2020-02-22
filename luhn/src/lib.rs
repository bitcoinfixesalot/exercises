/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

        if code.iter().all(|c| c.is_ascii_digit()) && code.len() > 1 {
        let sum = code
            .iter()
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
    } else {
        false
    }
}
