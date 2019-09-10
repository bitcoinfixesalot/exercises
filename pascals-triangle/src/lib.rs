pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = PascalsTriangle { rows: Vec::new() };
        
        let mut value = 1;
        for row_index in 0..row_count {
            let mut row = Vec::new();
            for position in 0..(row_index + 1) {
                if position != 0 {
                    value = value * (row_index - position + 1) / position;
                }
                row.push(value);
            }
            triangle.rows.push(row);
        }
        triangle
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
