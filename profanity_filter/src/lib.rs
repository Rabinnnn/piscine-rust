pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Message { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.trim().is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &Message) -> (bool, &str) {
    match message.send_ms() {
        Some(msg) => (true, msg),
        None => (false, "ERROR: illegal"),
    }
}
