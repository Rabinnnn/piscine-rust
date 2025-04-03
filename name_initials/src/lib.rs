pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter()
        .map(|name| {
            name.split_whitespace()
                .filter_map(|word| word.chars().next())
                .map(|c| format!("{}.", c.to_uppercase()))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect()
}
