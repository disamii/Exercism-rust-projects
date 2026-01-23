pub struct PascalsTriangle {
    row_count: u32,
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::new();

        for r in 0..row_count {
            let mut row = vec![1; (r + 1) as usize]; 
            if r >= 2 {
                let prev_row = &triangle[(r - 1) as usize];
                for i in 1..r as usize {
                    row[i] = prev_row[i - 1] + prev_row[i];
                }
            }
            triangle.push(row);
        }

        PascalsTriangle { row_count, triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}
