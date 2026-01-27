use std::collections::HashMap;

/// Compute the Scrabble score for a word.

pub fn score(word: &str) -> u64 {
    let mut points:u64 = 0;
    let letter_values: HashMap<Vec<char>, u64> = HashMap::from([
        (vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'], 1),
        (vec!['D', 'G'], 2),
        (vec!['B', 'C', 'M', 'P'], 3),
        (vec!['F', 'H', 'V', 'W', 'Y'], 4),
        (vec!['K'], 5),
        (vec!['J', 'X'], 8),
        (vec!['Q', 'Z'], 10),
    ]);
    for letter in word.chars() {
            let uppercase_letter = letter.to_ascii_uppercase();

        let score:u64 = letter_values
            .iter()
            .find(|(letters, _)| letters.contains(&uppercase_letter))
            .map(|(_, &v)| v)
            .unwrap_or(0);
        points+=score;
    }
    points

}
