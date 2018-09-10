use std::{collections::HashMap, fmt};
use types::buttons::Buttons;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    name: String,
    pub art: String,
    pub buttons: Buttons,
}

impl Tile {
    pub fn new(name: String, art: String) -> Self {
        Tile {
            name,
            art,
            buttons: Buttons::new(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)
    }
}

pub type TileID = u32;
pub type Tiles = HashMap<TileID, Tile>;
