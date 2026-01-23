/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut seen = [false; 26];

    for c in sentence.chars() {
        if c.is_ascii_alphabetic() {
            let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            seen[index] = true;
        }
    }

    seen.iter().all(|&v| v)
}
