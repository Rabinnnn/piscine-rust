pub fn scytale_cipher(s: String, i: u32) -> String {
    // Return original string if i is 1 or greater than or equal to string length
    if i as usize >= s.chars().count() || i == 1 {
        return s.to_string();
    }

    // Calculate number of columns (width) based on ceiling of length / i
    let width = (s.chars().count() as f64 / i as f64).ceil() as usize;

    // Create a 2D vector (table) with i rows and 'width' columns, initialized with spaces
    let mut table = vec![vec![' '; width]; i as usize];

    // Fill the table column-wise with characters from the input string
    for (pos, element) in s.chars().enumerate() {
        let col = pos % i as usize;   // Determine column index
        let row = pos / i as usize;   // Determine row index

        table[col][row] = element;    // Place character at the calculated position
    }

    // Flatten the table row-wise and collect characters into a single string
    table
        .iter()
        .flatten()
        .collect::<String>()
        .trim_end()     // Remove trailing spaces
        .to_string()    // Convert to String
}