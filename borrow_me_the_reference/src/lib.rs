pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    
    while i < chars.len() {
        match chars[i] {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            }
            '+' => {
                i += 1;
            }
            c => result.push(c),
        }
        i += 1;
    }
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() == 3 {
            if let (Ok(lhs), Ok(rhs)) = (tokens[0].parse::<i32>(), tokens[2].parse::<i32>()) {
                let result = match tokens[1] {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    _ => continue,
                };
                *expr = result.to_string();
            }
        }
    }
}
