pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut nums = Vec::new();
    if len == 0 || len > digits.len() {
        return nums;
    }

        for start in 0..digits.len() {
            let substring = &digits[start..start + len];
            nums.push(substring.to_string())
        }
    
    return nums;
}
