enum Direction {
    Right,
    Left,
    Down,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut direction = Direction::Right;
    let mut col: i32 = 0;
    let mut row: i32 = 0;

    for i in 1..((size * size) + 1) {
        match direction {
            Direction::Right
                if col as u32 > size - 1 || matrix[row as usize][col as usize] != 0 =>
            {
                direction = Direction::Down;
                col -= 1;
                row += 1;
            }
            Direction::Down if row as u32 > size - 1 || matrix[row as usize][col as usize] != 0 => {
                direction = Direction::Left;
                col -= 1;
                row -= 1;
            }
            Direction::Left if col < 0 || matrix[row as usize][col as usize] != 0 => {
                direction = Direction::Up;
                col += 1;
                row -= 1;
            }
            Direction::Up if row < 0 || matrix[row as usize][col as usize] != 0 => {
                direction = Direction::Right;
                col += 1;
                row += 1;
            }
            _ => (),
        }
        matrix[row as usize][col as usize] = i;

        match direction {
            Direction::Right => col += 1,
            Direction::Down => row += 1,
            Direction::Left => col -= 1,
            Direction::Up => row -= 1,
        }
    }

    matrix
}
