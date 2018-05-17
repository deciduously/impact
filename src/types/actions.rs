use super::super::{Model, Msg};
use types::{flags::{BoolFlag, FloatFlag, IntFlag}, messages::Message, resources::Resource,
            time::Time};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Noop, // Just wait a tick
    AddMessage(String),
    SetBoolFlag(BoolFlag),
    ClearBoolFlag(BoolFlag),
    SetResourceValue(Resource, i32),
    AddResourceValue(Resource, i32),
    SetIntFlag(IntFlag, i32),
    SetFloatFlag(FloatFlag, i32),
}

impl Action {
    pub fn perform(&self, model: &mut Model) {
        use self::Action::*;
        match self {
            Noop => {}
            AddResourceValue(resource, delta) => {
                // TODO add min/maxes, and check here
                let r = model.resource_values.entry(*resource).or_insert(0.0);
                *r += *delta as f64;
            }
            SetBoolFlag(f) => {
                model.bool_flags.insert(*f, true);
            }
            ClearBoolFlag(f) => {
                model.bool_flags.insert(*f, false);
            }
            SetResourceValue(resource, amt) => {
                model.resource_values.insert(*resource, *amt as f64);
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
        };
    }
}

// Also use references/boxes better?  maybe avoid the clone?
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
    pub tick: Time,
    pub action: Action,
}

impl TimeAction {
    pub fn new(tick: u64, action: Action) -> Self {
        TimeAction {
            tick: Time::from(tick),
            action,
        }
    }
}
// fn Perform
