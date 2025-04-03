pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter()
        .map(|name| {
            let mut result = String::new();
            for word in name.split_whitespace() {
                if let Some(c) = word.chars().next() {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push(c.to_ascii_uppercase());
                    result.push('.');
                }
            }
            result
        })
        .collect()
}
