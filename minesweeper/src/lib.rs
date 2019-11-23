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
    //let mut count = 0;
    if &minefield[i][j..(j + 1)] != " " {
        return None;
    }
    let height = minefield.len() as i8;
    let width = minefield[i].len() as i8;
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
    //             count += 1
    //         }
    //     }
    // }
    let count: u8 = NEIGBOURS.iter()
    .map(|&(x,y)| (x + i as i8, y + j as i8))
    .filter(|&(x,y)| (x >=0 && x < width) && (y >= 0 && y < height) )
    .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*').count() as u8;
    //.fold(0, |sum, (x,y)| sum + is_mine(&minefield, x as usize, y as usize) as u8);
    Some(count)
}

static NEIGBOURS: &'static [(i8, i8)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];