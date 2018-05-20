use std::collections::HashMap;
use types::resources::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoolFlag {
    OxygenMonitor, // controls corresponding button visibility
    LeakyTank,
    PowerRegen,
}

pub type BoolFlags = HashMap<BoolFlag, bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFlag {
    ResourceDelta(Resource, i64),
}

pub type IntFlags = HashMap<IntFlag, i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFlag {
    NotYet,
}

pub type FloatFlags = HashMap<FloatFlag, f64>;
