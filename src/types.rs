use std::{fmt, collections::HashMap};

// Actions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    EndTurn,
    AddMessage(String),
    //SetBoolFlag(BoolFlag),
    //ClearBoolFlag(BoolFlag),
    //SetResourceValue(Resource, i32),
    AddResourceValue(Resource, i32),
    //AddIntFlag(IntFlag, i32),
    //SetFloatFlag(FloatFlag),
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

pub type Resources = HashMap<Resource, i32>;

// Buttons

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Button {
    EndTurn,
}

// Time
pub type Tick = u64;

//User-facing Messages

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    time: Tick,
    content: String,
}

impl Message {
    pub fn new(content: String, time: Tick) -> Self {
        Message { content, time }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.time, self.content)
    }
}
