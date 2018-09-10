use std::fmt;
use types::time::Time;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub time: Time,
    pub content: String,
}

impl Message {
    pub fn new(content: String, time: Time) -> Self {
        Message { content, time }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.time, self.content)
    }
}
