use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Oxygen,
}

pub type Resources = HashMap<Resource, i32>;
