use std::collections::HashMap;

pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Message {
        Message { content: ms, user: u }
    }

    pub fn send_ms(&self) -> Option<&str> {
        let lower = self.content.to_lowercase();
        if lower.is_empty() || lower.contains("stupid") {
            None
        } else {
            Some(self.content.as_str())
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(text) => (true, text),
        None => (false, "ERROR: illegal"),
    }
}
