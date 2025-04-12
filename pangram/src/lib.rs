use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let letters: HashSet<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    letters.len() == 26
}
