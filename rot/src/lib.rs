pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let rotated = (c as u8 - base + (key.rem_euclid(26) as u8)) % 26 + base;
                rotated as char
            } else {
                c
            }
        })
        .collect()
}
