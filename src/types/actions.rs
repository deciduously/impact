use super::super::Model; // joke
use types::{messages::Message, resources::Resource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    AddMessage(String),
    //SetBoolFlag(BoolFlag),
    //ClearBoolFlag(BoolFlag),
    SetResourceValue(Resource, i32),
    AddResourceValue(Resource, i32),
    //AddIntFlag(IntFlag, i32),
    //SetFloatFlag(FloatFlag),
}

impl Action {
    pub fn perform(self, model: &mut Model) {
        use self::Action::*;
        match self {
            AddResourceValue(resource, delta) => {
                // TODO add min/maxes, and check here
                let r = model.resource_values.entry(resource).or_insert(0.0);
                *r += delta as f64;
            }
            SetResourceValue(resource, amt) => {
                model.resource_values.insert(resource, amt as f64);
            }
            AddMessage(message) => {
                model.messages.push(Message::new(message, model.tick));
            }
        };
        model.tick += 1; // TODO Model::tick() which will apply transformers and then tick fwd
    }
}

// TODO timeactions which fire at a given tick - can limp in with Action
