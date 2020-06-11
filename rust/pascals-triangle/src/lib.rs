pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).fold(Vec::new(), |mut rows, row_index| {
            let mut row = vec![1];

            if let Some(prev_row) = rows.last() {
                for i in 1..row_index as usize {
                    row.push(prev_row[i - 1] + prev_row[i]);
                }

                if let Some(&last) = prev_row.last() {
                    row.push(last);
                }
            }

            rows.push(row);
            rows
        })
    }
}
