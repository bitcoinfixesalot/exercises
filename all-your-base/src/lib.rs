#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        Err(Error::InvalidInputBase)
    } else if to_base <= 1 {
        Err(Error::InvalidOutputBase)
    } else {
        convert_from_base(number, from_base).and_then(|n| Ok(convert_to_base(n, to_base)))
    }
}

fn convert_from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    let mut result: u32 = 0;
    for &digit in number {
        if digit >= base {
            return Err(Error::InvalidDigit(digit));
        } else {
            result = result * base + digit;
        }
    }
    Ok(result)
}

fn convert_to_base(number: u32, base: u32) -> Vec<u32> {
    let mut n: u32 = number;
    let mut result: Vec<u32> = Vec::new();
    while n > 0 {
        result.insert(0, n % base);
        n /= base;
    }
    result
}
