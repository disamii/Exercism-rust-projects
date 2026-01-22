/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut checksum=0;
    let isbn: String = isbn.chars().filter(|&c| c != '-').collect(); // remove dashes
    if isbn.len() != 10 {
        return false;
    }
    for (i, c) in isbn.chars().enumerate() {
        let value = if i == 9 && c == 'X' {
            10 
        } else {
            match c.to_digit(10) {
                Some(d) => d,
                None => return false, // invalid character
            }
        };
        checksum += value * (10 - i as u32);
    }

    checksum % 11 == 0
}
