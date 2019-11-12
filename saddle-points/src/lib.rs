pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for (row, row_vector) in input.iter().enumerate() {
        for (col, number) in row_vector.iter().enumerate() {
            if row_vector.iter().all(|a| a <= number) && input.iter().all(|a| a[col] >= *number) {
                saddle_points.push((row, col));
            }
        }
    }

    saddle_points
}
