pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut matrix = vec![vec![0u32; size]; size]; // u32 matrix

    let mut counter: u32 = 1; // counter is u32
    let mut top = 0;
    let mut bottom = size.saturating_sub(1);
    let mut left = 0;
    let mut right = size.saturating_sub(1);

    while (counter as usize) <= size * size {
        // Top row: left → right
        for col in left..=right {
            matrix[top][col] = counter;
            counter += 1;
        }
        top += 1;

        // Right column: top → bottom
        for row in top..=bottom {
            matrix[row][right] = counter;
            counter += 1;
        }
        if right == 0 { break; }
        right = right.saturating_sub(1);

        // Bottom row: right → left
        if top <= bottom {
            for col in (left..=right).rev() {
                matrix[bottom][col] = counter;
                counter += 1;
            }
            if bottom == 0 { break; }
            bottom = bottom.saturating_sub(1);
        }

        // Left column: bottom → top
        if left <= right {
            for row in (top..=bottom).rev() {
                matrix[row][left] = counter;
                counter += 1;
            }
            left += 1;
        }
    }

    matrix
}