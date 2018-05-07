use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoolFlag {
    OxygenMonitor,
}

pub type BoolFlags = HashMap<BoolFlag, bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFlag {
    NotYet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFlag {
    OxygenDepletion,
}
