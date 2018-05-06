use std::collections::HashMap;

// Actions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    ActionEndTurn,
    ActionAddMsg(String),
    ActionSetBoolFlag(BoolFlag),
    ActionClearBoolFlag(BoolFlag),
    ActionSetResourceValue(Resource, i32),
    ActionAddResourceValue(Resource, i32),
    ActionAddIntFlag(IntFlag, i32),
    ActionSetFloatFlag(FloatFlag),
}

// Flags

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoolFlag {
    OxygenMonitor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntFlag {
    NotYet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatFlag {
    OxygenDepletion,
}

// Resources

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Oxygen,
}

pub type Resources = HashMap<Resource, u32>;

// Buttons

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Button {
    EndTurn,
}
