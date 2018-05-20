use super::super::{Model, Msg};
use types::{buttons::Button, flags::{BoolFlag, FloatFlag, IntFlag}, messages::Message,
            resources::Resource, tiles::{Tile, TileID}, time::Time};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Noop, // Just wait a tick
    AddMessage(String),
    SetBoolFlag(BoolFlag),
    ClearBoolFlag(BoolFlag),
    SetResourceValue(Resource, i64),
    AddResourceValue(Resource, i64),
    SetIntFlag(IntFlag, i64),
    SetFloatFlag(FloatFlag, i64),
    EnableButton(TileID, Button),
    DisableButton(TileID, Button),
    AddTile(TileID, Tile),
}

impl Action {
    pub fn perform(&self, model: &mut Model) {
        use self::Action::*;
        match self {
            Noop => {}
            AddResourceValue(resource, delta) => {
                // TODO add min/maxes, and check here
                let r = model.resource_values.entry(*resource).or_insert(0);
                *r += *delta;
            }
            SetBoolFlag(f) => {
                model.bool_flags.insert(*f, true);
            }
            ClearBoolFlag(f) => {
                model.bool_flags.insert(*f, false);
            }
            SetResourceValue(resource, amt) => {
                model.resource_values.insert(*resource, *amt);
            }
            AddMessage(message) => {
                model
                    .messages
                    .push(Message::new(message.to_string(), &model.time));
            }
            SetIntFlag(f, amt) => {
                model.int_flags.insert(*f, *amt);
            }
            SetFloatFlag(f, amt) => {
                model.float_flags.insert(*f, *amt as f64);
            }
            EnableButton(tid, button) => {
                // grab tile first
                let mut t = model.tiles.get(tid).unwrap().clone();
                t.buttons.insert(button.clone(), true);
                model.tiles.insert(*tid, t); // push it back to the model
            }
            DisableButton(tid, button) => {
                let mut t = model.tiles.get(tid).unwrap().clone();
                t.buttons.insert(button.clone(), false);
                model.tiles.insert(*tid, t);
            }
            AddTile(tid, tile) => {
                model.tiles.insert(*tid, tile.clone());
            }
        };
    }
}

// TODO use references/boxes better?  maybe avoid the clone?
pub fn msg_from_actions(actions: Vec<Action>) -> Msg {
    if actions.len() == 0 {
        return Msg::PerformAction(self::Action::Noop);
    } else if actions.len() == 1 {
        return Msg::PerformAction(actions[0].clone());
    }

    let mut pas = Vec::new();
    for a in actions.iter() {
        pas.push(Msg::PerformAction(a.clone()));
    }
    Msg::Bulk(pas)
}

#[derive(Debug, Clone)]
pub struct TimeAction {
    pub time: Time,
    pub action: Action, //Vec<Action>?
}

impl TimeAction {
    pub fn new(tick: u64, action: Action) -> Self {
        TimeAction {
            time: Time::from(tick),
            action,
        }
    }
}

pub fn apply_timeactions(model: &mut Model) {
    // TODO where the heck should these live?
    // I don't want to reallocate the whole thing every time...
    let timeactions = vec![
        TimeAction::new(1, Action::EnableButton(0, Button::ActivateOxygen)),
        TimeAction::new(15, Action::AddMessage("It's been 15 SECONDS".to_string())),
    ];
    for ta in timeactions.iter() {
        if ta.time == model.time {
            ta.action.perform(model);
        }
    }
}
