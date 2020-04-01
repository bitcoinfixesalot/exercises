pub fn count(lines: &[&str]) -> u32 {
    lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|&(_, &c)| is_corner(c))
                .map(|(col, _)| count_rectangles_with_corner(lines, row, col))
                .sum::<u32>()
        })
        .sum()
}

fn count_rectangles_with_corner(lines: &[&str], row: usize, col1: usize) -> u32 {
    if let Some(line) = lines.get(row) {
        line.as_bytes()
            .iter()
            .enumerate()
            .skip(col1 + 1)
            .take_while(|&(_, &c)| is_horizontal(c))
            .filter(|&(_, &c)| is_corner(c))
            .map(|(col2, _)| count_rectangles_with_base(lines, row, col1, col2))
            .sum()
    } else {
        0
    }
}

fn count_rectangles_with_base(lines: &[&str], row: usize, col1: usize, col2: usize) -> u32 {
    lines
        .iter()
        .map(|line| line.as_bytes())
        .skip(row + 1)
        .take_while(|line| line.get(col1).map(|&c| is_vertical(c)) == Some(true))
        .take_while(|line| line.get(col2).map(|&c| is_vertical(c)) == Some(true))
        .filter(|line| {
            line.iter()
                .skip(col1)
                .take(col2 - col1)
                .all(|&c| is_horizontal(c))
        })
        .count() as u32
}

fn is_corner(symbol: u8) -> bool {
    symbol == b'+'
}

fn is_vertical(c: u8) -> bool {
    c == b'|' || c == b'+'
}

fn is_horizontal(c: u8) -> bool {
    c == b'-' || c == b'+'
}
