pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .filter_map(|word| word.chars().next()) // Get first letter of each word
                .map(|c| format!("{}.", c)) // Format each letter as "X."
                .collect::<Vec<_>>() // Collect into a vector
                .join(" ") // Join with spaces, e.g., "H. P."
        })
        .collect()
}