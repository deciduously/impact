use std::{collections::HashMap, fmt};
use types::buttons::{ButtonID, Buttons};

// Tiles still hold buttons, but all buttons are displayed in a global action bar

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    name: String,
    pub art: String,
    pub buttons: Buttons,
}

impl Tile {
    pub fn new(name: String, art: String, buttons: Vec<ButtonID>) -> Self {
        Tile { name, art, buttons }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self { name: "Wasteland".into(), art: "........".into(), buttons: vec![] }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)
    }
}

pub type TileID = u32;
pub type Tiles = HashMap<TileID, Tile>;

pub fn defined_tiles(id: TileID) -> Option<Tile> {
    match id {
        0 => Some(Tile::new("Ship".into(), "..::^::..".into(), vec![0])),
        1 => Some(Tile::new(
            "Field".into(),
            ".......!!!!!.....".into(),
            Vec::new(),
        )),
        _ => None,
    }
}
