pub fn get_diamond(c: char) -> Vec<String> {
    let mut lines = Vec::new();

    let mut current_letter = 'A';

    while current_letter <= c {
        let skip = " ".repeat(c as usize - current_letter as usize);

        if current_letter == 'A' {
            lines.push(format!("{}{}{}", skip, current_letter, skip));
        } else {
            let middle = " ".repeat(2 * (current_letter as usize - 'A' as usize) - 1);
            lines.push(format!("{}{}{}{}{}", skip, current_letter, middle, current_letter, skip));
        }

        current_letter = (current_letter as u8 + 1) as char;
    }

    lines.iter()
        .chain(lines.iter().rev().skip(1))
        .map(|s| s.to_string())
        .collect()
}
