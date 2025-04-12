pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();

    for word in text.split_whitespace() {
        let lower = word.to_lowercase();
        let chars: Vec<char> = lower.chars().collect();

        if vowels.contains(&chars[0]) {
            result.push_str(&format!("{}ay", word));
        } else if chars.len() > 2 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
            // handle cases like "square" => "aresquay"
            result.push_str(&format!("{}{}ay", &word[3..], &word[..3]));
        } else {
            // find index of first vowel
            let mut first_vowel_idx = 0;
            for (i, c) in chars.iter().enumerate() {
                if vowels.contains(c) {
                    first_vowel_idx = i;
                    break;
                }
            }
            result.push_str(&format!(
                "{}{}ay",
                &word[first_vowel_idx..],
                &word[..first_vowel_idx]
            ));
        }
        result.push(' ');
    }

    result.trim_end().to_string()
}
