#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ if span > string_digits.len() => Err(Error::SpanTooLong),
        _ => {
            let string_digits: Vec<u64> = string_digits
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .and_then(|n| Some(n as u64))
                        .ok_or_else(|| Error::InvalidDigit(c))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(string_digits
                .windows(span)
                .map(|arr| arr.iter().product())
                .max()
                .unwrap())
        }
    }
}
