pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();
    
    // Convert input to Vec<Vec<char>> for easier manipulation
    let grid: Vec<Vec<char>> = garden.iter().map(|row| row.chars().collect()).collect();

    let mut result = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut new_row = String::with_capacity(cols);
        for c in 0..cols {
            if grid[r][c] == '*' {
                new_row.push('*');
            } else {
                // Count adjacent flowers
                let mut count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue; // skip self
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                            if grid[nr as usize][nc as usize] == '*' {
                                count += 1;
                            }
                        }
                    }
                }
                if count == 0 {
                    new_row.push(' ');
                } else {
                    new_row.push(char::from_digit(count, 10).unwrap());
                }
            }
        }
        result.push(new_row);
    }

    result
}
