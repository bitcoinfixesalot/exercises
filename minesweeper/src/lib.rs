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
    //let mut count = 0;

    // for x in 0..3 {
    //     for y in 0..3 {
    //         if i + 1 < x
    //             || i + 1 - x >= minefield.len()
    //             || j + 1 < y
    //             || j + 1 - y >= minefield[i].len()
    //             || (x == 1 && y == 1)
    //         {
    //             continue;
    //         } else if &minefield[i + 1 - x][(j + 1 - y)..(j + 2 - y)] == "*" {
    //             count += 1;
    //         }
    //     }
    // }

    let count= neighbours(i as i32, j as i32).iter().filter_map(|&(x, y)| {
        let row: &str = minefield.get(x as usize)?;
        row.chars().nth(y as usize)
    }).map(|c| match c{ '*' => 1, _=> 0,}).sum();

    Some(count)
}

fn neighbours(x: i32, y: i32) -> [(i32, i32); 8] {
    [
        (x-1, y-1), (x, y-1), (x+1, y-1),
        (x-1, y),               (x+1, y),
        (x-1, y+1), (x, y+1), (x+1, y+1)
    ]
}
