use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    time: u64,
    content: String,
}

impl Message {
    pub fn new(content: String, time: u64) -> Self {
        Message { content, time }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.time, self.content)
    }
}
