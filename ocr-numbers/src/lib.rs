use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let rows = input.lines().collect::<Vec<&str>>();
    if rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    } else if rows.iter().any(|row| row.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(rows[0].len()));
    }

    let letters = vec![
        (" _ | ||_|   ", "0"),
        ("     |  |   ", "1"),
        (" _  _||_    ", "2"),
        (" _  _| _|   ", "3"),
        ("   |_|  |   ", "4"),
        (" _ |_  _|   ", "5"),
        (" _ |_ |_|   ", "6"),
        (" _   |  |   ", "7"),
        (" _ |_||_|   ", "8"),
        (" _ |_| _|   ", "9"),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, &str>>();

    let mut result = String::new();
    for letter_row in rows.chunks(4) {
        for col in (0..rows[0].len()).step_by(3) {
            let letter: String = letter_row
                .iter()
                .flat_map(|row| row[col..col + 3].chars())
                .collect();
            result += letters.get(letter.as_str()).unwrap_or(&"?");
        }
        result.push(',');
    }
    result.pop();

    Ok(result)
}
