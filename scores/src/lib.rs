pub fn score(input: &str) -> u64 {
    input
        .to_uppercase()
        .chars()
        .filter_map(|c| match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => Some(1),
            'D' | 'G' => Some(2),
            'B' | 'C' | 'M' | 'P' => Some(3),
            'F' | 'H' | 'V' | 'W' | 'Y' => Some(4),
            'K' => Some(5),
            'J' | 'X' => Some(8),
            'Q' | 'Z' => Some(10),
            _ => None, // ignore characters not in the scoring table
        })
        .map(|v| v as u64)
        .sum()
}
