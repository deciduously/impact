use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Chutzpah,
    Oxygen,
    Power,
}

pub type Resources = HashMap<Resource, i64>;
