fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();

    // Calculate number of columns needed
    let columns = (len as f32 / i as f32).ceil() as usize;

    // Pad the message with spaces if needed
    let mut padded_chars = chars.clone();
    while padded_chars.len() < i * columns {
        padded_chars.push(' ');
    }

    // Read column-wise
    let mut result = String::new();
    for col in 0..columns {
        for row in 0..i {
            let index = row * columns + col;
            if index < padded_chars.len() {
                result.push(padded_chars[index]);
            }
        }
    }

    result
}
