use std::{fmt, ops::Rem};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Time {
    pub seconds: u64,
}

impl Time {
    pub fn new() -> Time {
        Time { seconds: 0 }
    }
    pub fn increment(&mut self) {
        self.seconds += 1;
    }
    pub fn from(seconds: u64) -> Time {
        Time { seconds }
    }
}

// TODO only show up to the highest amount
// get the string first, pass the pre-found thing to the write! call, etc
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.seconds;
        write!(
            f,
            "{}d{}h{}m{}s",
            s / (60 * 60 * 24),
            s / (60 * 60),
            s / 60,
            u64::rem(s, 60)
        )
    }
}
