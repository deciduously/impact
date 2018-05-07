pub mod actions;
pub mod buttons;
pub mod flags;
pub mod messages;
pub mod resources;

use self::actions::Action;

pub enum Msg {
    PerformAction(Action),
    Bulk(Vec<Msg>),
}

pub type Tick = u64; //TODO chrono?  pending wasm32-unknown-unknown support
