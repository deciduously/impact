pub mod actions;
pub mod buttons;
pub mod flags;
pub mod messages;
pub mod resources;
pub mod time;

use self::actions::Action;

pub enum Msg {
    Tick,
    PerformAction(Action),
    Bulk(Vec<Msg>),
}
