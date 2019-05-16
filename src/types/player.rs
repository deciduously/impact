use crate::types::tiles::TileID;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub current_tile: TileID,
    pub name: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            current_tile: 0,
            name: "Spiff".into(),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}