use std::collections::HashMap;
use types::{resources::Resource, transformers::Transformer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoolFlag {
    OxygenMonitor, // controls corresponding button visibility
    LeakyTank,
    PowerRegen,
}

impl BoolFlag {
    pub fn transformer(&self) -> Option<Transformer> {
        use self::BoolFlag::*;
        match self {
            OxygenMonitor => None,
            LeakyTank => Some(Transformer::LeakyTank),
            PowerRegen => Some(Transformer::PowerRegen),
        }
    }
}

pub type BoolFlags = HashMap<BoolFlag, bool>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFlag {
    ResourceDelta(Resource),
}

pub type IntFlags = HashMap<IntFlag, i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFlag {
    NotYet,
}

pub type FloatFlags = HashMap<FloatFlag, f64>;
