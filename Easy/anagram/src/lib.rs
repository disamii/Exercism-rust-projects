use std::collections::HashSet;


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_lower = word.to_lowercase();

    for &candidate in possible_anagrams {
        if candidate.eq_ignore_ascii_case(word) {
            continue;
        }

        let mut chars_candidate: Vec<char> = candidate.to_lowercase().chars().collect();
        let mut chars_word: Vec<char> = word_lower.chars().collect();
        chars_candidate.sort_unstable();
        chars_word.sort_unstable();

        if chars_candidate == chars_word {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
