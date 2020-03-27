const ZERO: char = '0';
const ONE: char = '1';
pub fn number(user_number: &str) -> Option<String> {
    let mut user_number: Vec<char> = user_number.chars().filter(|c| c.is_numeric()).collect();

    if user_number.len() == 11 && user_number[0] == ONE {
        user_number.remove(0);
    }

    match user_number.len() {
        10 if user_number[0] == ZERO
            || user_number[0] == ONE
            || user_number[3] == ZERO
            || user_number[3] == ONE =>
        {
            None
        }
        10 => Some(user_number.iter().collect()),
        _ => None,
    }
}
