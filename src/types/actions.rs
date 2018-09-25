use super::super::{Model, Msg};
use types::{
    buttons::{Button, ButtonID},
    flags::BoolFlag,
    messages::Message,
    resources::Resource,
    tiles::{defined_tiles, TileID},
    time::Time,
    transformers::Transformation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Noop, // Just wait a tick
    AddMessage(String),
    SetBoolFlag(BoolFlag),
    ClearBoolFlag(BoolFlag),
    SetResourceValue(Resource, i64),
    AddResourceValue(Resource, i64),
    AddResourceDelta(Resource, i64),
    //SetIntFlag(IntFlag, i64),
    //SetFloatFlag(FloatFlag, i64),
    EnableButton(ButtonID),
    DisableButton(ButtonID),
    AddTile(TileID),
}

impl Action {
    pub fn perform(&self, model: &mut Model) {
        use self::Action::*;
        match self {
            Noop => {}
            SetBoolFlag(f) => {
                model.bool_flags.insert(*f, true);
                //apply delta
                if let Some(tf) = f.transformer() {
                    for eff in tf.effects() {
                        match eff {
                            Transformation::Generate(r, delta) => {
                                Action::AddResourceDelta(r, delta).perform(model);
                            }
                            Transformation::Consume(r, delta) => {
                                Action::AddResourceDelta(r, -delta).perform(model);
                            }
                        }
                    }
                }
            }
            ClearBoolFlag(f) => {
                model.bool_flags.insert(*f, false);
                //remove delta
                // TODO: POTENTIAL BUG
                // you should check if it's already false or not
                if let Some(tf) = f.transformer() {
                    for eff in tf.effects() {
                        match eff {
                            Transformation::Generate(r, delta) => {
                                Action::AddResourceDelta(r, -delta).perform(model);
                            }
                            Transformation::Consume(r, delta) => {
                                Action::AddResourceDelta(r, delta).perform(model);
                            }
                        }
                    }
                }
            }
            SetResourceValue(resource, amt) => {
                model.resource_values.insert(*resource, (*amt, 0));
            }
            AddResourceValue(resource, delta) => {
                // TODO add min/maxes, and check here
                let r = model.resource_values.entry(*resource).or_insert((0, 0));
                r.0 += *delta;
            }
            AddResourceDelta(resource, delta) => {
                let r = model.resource_values.entry(*resource).or_insert((0, 0));
                r.1 += *delta;
            }
            AddMessage(message) => {
                model
                    .messages
                    .push(Message::new(message.to_string(), model.time));
            }
            //SetIntFlag(f, amt) => {
            //    model.int_flags.insert(*f, *amt);
            //}
            //SetFloatFlag(f, amt) => {
            //    model.float_flags.insert(*f, *amt as f64);
            //}
            EnableButton(bid) => {
                if Button::by_index(*bid).is_some() {
                    model.buttons.push(*bid);
                }
            }
            DisableButton(bid) => {
                if Button::by_index(*bid).is_some() {
                    let mut button_idx: i32 = -1;
                    for i in 0..model.buttons.len() {
                        if model.buttons[i as usize] == *bid {
                            button_idx = i as i32;
                        }
                    }
                    if button_idx >= 0 {
                        model.buttons.remove(button_idx as usize);
                    }
                }
            }
            AddTile(tid) => {
                let new_tile = defined_tiles(*tid).unwrap();
                model.tiles.insert(*tid, new_tile.clone());
                let new_buttons = new_tile.buttons.clone();
                for b in new_buttons {
                    model.buttons.push(b);
                }
            }
        };
    }
}

// TODO use references/boxes better?  maybe avoid the clone?
pub fn msg_from_actions(actions: &[Action]) -> Msg {
    if actions.is_empty() {
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
        TimeAction::new(1, Action::EnableButton(1)),
        TimeAction::new(15, Action::AddMessage("It's been 15 SECONDS".to_string())),
    ];
    for ta in timeactions.iter() {
        if ta.time == model.time {
            ta.action.perform(model);
        }
    }
}
