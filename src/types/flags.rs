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

pub type IntFlags = HashMap<IntFlag, i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFlag {
    OxygenDepletion,
}

pub type FloatFlags = HashMap<FloatFlag, f64>;
