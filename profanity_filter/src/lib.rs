struct Message {
    content: String,
    user: String,
}

impl Message {
    fn new(content: String, user: String) -> Message {
        Message { content, user }
    }

    fn send_ms(&self) -> Option<String> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content.clone())
        }
    }
}

fn check_ms(ms: &Message) -> (bool, String) {
    match ms.send_ms() {
        None => (false, "ERROR: illegal".to_string()),
        Some(content) => (true, content),
    }
}