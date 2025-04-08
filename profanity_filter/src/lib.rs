pub struct Message<'a> {
    content: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }

    pub fn send_ms(&self) -> Option<&'a str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message);
    match msg.send_ms() {
        Some(valid) => Ok(valid),
        None => Err("ERROR: illegal"),
    }
}