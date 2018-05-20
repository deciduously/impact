use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Chutzpah,
    Oxygen,
    Power,
}

// tuple stores Amount, Delta
pub type Resources = HashMap<Resource, (i64, i64)>;
