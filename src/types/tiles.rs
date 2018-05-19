#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    id: u32,
    pub name: String,
}

impl Tile {
    pub fn new(id: u32, name: String) -> Self {
        Tile { id, name }
    }
}

pub type Tiles = Vec<Tile>;
