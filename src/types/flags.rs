use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoolFlag {
    OxygenMonitor, // controls corresponding button visibility
    LeakyTank,
    PowerRegen,
}

pub type BoolFlags = HashMap<BoolFlag, bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFlag {
    NotYet,
}

pub type IntFlags = HashMap<IntFlag, i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFlag {
    OxygenDepletion, // redundant?
}

pub type FloatFlags = HashMap<FloatFlag, f64>;
