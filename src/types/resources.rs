use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Oxygen,
    Power,
}

pub type Resources = HashMap<Resource, f64>;
