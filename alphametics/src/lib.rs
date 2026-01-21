use std::collections::{HashMap, HashSet};


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // 1. Clean input and split
    let input = input.replace(" ", "");
let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        return None;
    }

    let left_words: Vec<&str> = parts[0].split('+').collect();
    let right_word = parts[1];

    // 2. Collect unique letters
    let mut letter_set: HashSet<char> = HashSet::new();
    for word in left_words.iter().chain(std::iter::once(&right_word)) {
        for c in word.chars() {
            letter_set.insert(c);
        }
    }
    if letter_set.len() > 10 {
        return None; // Impossible: more letters than digits
    }

    let ordered_letters: Vec<char> = letter_set.into_iter().collect();

    // 3. Identify leading letters (cannot be 0)
    let mut leading_letters: HashSet<char> = HashSet::new();
    for word in left_words.iter().chain(std::iter::once(&right_word)) {
        if let Some(c) = word.chars().next() {
            leading_letters.insert(c);
        }
    }

    // 4. Start backtracking search
    let mut mapping = HashMap::new();
return solve_recursive(
    &ordered_letters,
    &left_words,
    right_word,
    &leading_letters,
    &mut mapping,
);

}

fn solve_recursive(
    letters: &[char],
    left_words: &[&str],
    right_word: &str,
    leading_letters: &HashSet<char>,
    mapping: &mut HashMap<char, u8>,
) -> Option<HashMap<char, u8>> {
    if mapping.len() == letters.len() {
        if check_sum(left_words, right_word, mapping) {
            return Some(mapping.clone());
        } else {
            return None;
        }
    }

    
    // Next letter to assign
    let next_letter = letters[mapping.len()];

    // Try all digits 0..9
    for digit in 0..=9 {
        // Skip if already used
        if mapping.values().any(|&d| d == digit) {
            continue;
        }
        // Skip if leading letter is 0
        if digit == 0 && leading_letters.contains(&next_letter) {
            continue;
        }

        // Assign digit
        mapping.insert(next_letter, digit);
        // Recurse
        if let Some(solution) =
            solve_recursive(letters, left_words, right_word, leading_letters, mapping)
        {
            return Some(solution);
        }
        // Backtrack
        mapping.remove(&next_letter);
    }

    None
}

// Helper: convert a word to a number using mapping
fn word_to_number(word: &str, mapping: &HashMap<char, u8>) -> u64 {
    word.chars()
        .fold(0u64, |acc, c| acc * 10 + (*mapping.get(&c).unwrap() as u64))
}

// Helper: check if sum of left_words equals right_word
fn check_sum(left_words: &[&str], right_word: &str, mapping: &HashMap<char, u8>) -> bool {
    let sum_left: u64 = left_words
        .iter()
        .map(|word| word_to_number(word, mapping))
        .sum();
    let right = word_to_number(right_word, mapping);
    sum_left == right
}
