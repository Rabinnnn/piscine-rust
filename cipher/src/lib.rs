#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected: String = original
        .chars()
        .map(|c| {
            match c {
                'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
                'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
                _ => c,
            }
        })
        .collect();

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}