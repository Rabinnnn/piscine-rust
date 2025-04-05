pub fn first_subword(mut s: String) -> String {
    let mut chars = s.chars();
    let mut result = String::new();
    
    if let Some(first) = chars.next() {
        result.push(first);
    }
    
    for c in chars (
        if c.is_uppercase() || c == '_' {
            break;
        }
        result.push(c);
    )
    
    result
}
