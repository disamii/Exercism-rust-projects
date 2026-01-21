/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Remove spaces and validate characters
    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    if code.len() <= 1 || code.chars().any(|c| !c.is_digit(10)) {
        return false;
    }

    // Convert characters to digits in reverse order
    let digits: Vec<u32> = code.chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Compute the Luhn sum
    let sum: u32 = digits.iter().enumerate().map(|(i, &d)| {
        if i % 2 == 1 {  // double every second digit from the right
            let doubled = d * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else {
            d
        }
    }).sum();

    sum % 10 == 0
}
