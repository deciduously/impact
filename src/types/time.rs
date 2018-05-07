use std::{fmt, ops::Rem};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Time {
    seconds: u64,
}

impl Time {
    pub fn new() -> Time {
        Time { seconds: 0 }
    }
    pub fn increment(&mut self) {
        self.seconds += 1;
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (d, h, m, s) = dhms(self.seconds);
        write!(f, "{}d{}h{}m{}s", d, h, m, s)
    }
}

fn mins(s: u64) -> (u64, u64) {
    (s / 60, u64::rem(s, 60))
}

fn hms(s: u64) -> (u64, u64, u64) {
    let (m, s) = mins(s);
    (m / 60, m, s)
}

fn dhms(s: u64) -> (u64, u64, u64, u64) {
    let (h, m, s) = hms(s);
    (h / 24, h, m, s)
}
