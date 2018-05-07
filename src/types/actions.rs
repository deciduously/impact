use super::super::Model;
use types::{flags::BoolFlag, messages::Message, resources::Resource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Noop, // Just wait a tick
    AddMessage(String),
    SetBoolFlag(BoolFlag),
    ClearBoolFlag(BoolFlag),
    SetResourceValue(Resource, i32),
    AddResourceValue(Resource, i32),
    //AddIntFlag(IntFlag, i32),
    //SetFloatFlag(FloatFlag),
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
        };
        model.time.increment(); // TODO Model::tick() which will apply transformers and then tick fwd
    }
}

// TODO timeactions which fire at a given tick - can limp in with Action
