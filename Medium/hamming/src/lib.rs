/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut hamming = 0;
    for (char1, char2) in s1.chars().zip(s2.chars()) {
        if char1 != char2 {
            hamming += 1;
        }
    }
    Some(hamming)
}
