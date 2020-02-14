pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        result.push(String::new());
        for (j, col) in row.chars().enumerate() {
            match mines_count(minefield, i, j) {
                Some(0) => result[i].push(' '),
                Some(n) => result[i].push_str(&n.to_string()),
                None => result[i].push(col),
            }
        }
    }
    result
}

fn mines_count(minefield: &[&str], i: usize, j: usize) -> Option<u8> {
    if &minefield[i][j..(j + 1)] != " " {
        return None;
    }
    let mut count = 0;

    for x in 0..3 {
        for y in 0..3 {
            if i + 1 < x
                || i + 1 - x >= minefield.len()
                || j + 1 < y
                || j + 1 - y >= minefield[i].len()
                || (x == 1 && y == 1)
            {
                continue;
            } else if &minefield[i + 1 - x][(j + 1 - y)..(j + 2 - y)] == "*" {
                count += 1;
            }
        }
    }
    Some(count)
}
