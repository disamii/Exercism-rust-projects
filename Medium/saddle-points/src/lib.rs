pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // Handle empty matrix or empty rows
    if input.is_empty() || input[0].is_empty() {
        return Vec::new();
    }

    let rows = input.len();
    let cols = input[0].len();

    // Precompute maximum value for each row
    let row_max: Vec<u64> = input
        .iter()
        .map(|row| *row.iter().max().unwrap())
        .collect();

    // Precompute minimum value for each column
    let mut col_min = vec![u64::MAX; cols];
    for row in input {
        for (c, &value) in row.iter().enumerate() {
            col_min[c] = col_min[c].min(value);
        }
    }

    // Find saddle points
    let mut result = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            let value = input[r][c];
            if value == row_max[r] && value == col_min[c] {
                result.push((r, c));
            }
        }
    }

    result
}
