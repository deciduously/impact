use super::super::Model;
use types::{actions::Action, flags::BoolFlag, resources::Resource};

// Each transformer should have a corresponding BoolFlag
pub enum Transformer {
    LeakyTank,
    PowerRegen,
}

impl Transformer {
    pub fn effects(&self) -> Vec<Transformation> {
        use self::Transformation::*;
        match self {
            Transformer::LeakyTank => vec![Consume(Resource::Oxygen, 10)],
            Transformer::PowerRegen => {
                vec![Generate(Resource::Power, 2), Consume(Resource::Oxygen, 1)]
            }
        }
    }

    pub fn apply_transformer(&self, model: &mut Model) {
        for eff in self.effects().iter() {
            eff.apply_transformation(model);
        }
    }
}

pub enum Transformation {
    Generate(Resource, i64),
    Consume(Resource, i64),
}

impl Transformation {
    pub fn apply_transformation(&self, model: &mut Model) {
        use self::Transformation::*;
        match self {
            Generate(resource, delta) => {
                Action::AddResourceValue(*resource, *delta).perform(model);
            }
            Consume(resource, delta) => {
                Action::AddResourceValue(*resource, -(*delta)).perform(model);
            }
        }
    }
}

pub fn apply_transformers(model: &mut Model) {
    let bfs = model.bool_flags.clone(); // GROSS

    for (f, enabled) in bfs {
        match f {
            // this doesn't need to be a match - you're doing the same thing for any Transformer
            // how do I compare across enums?
            BoolFlag::LeakyTank => {
                if enabled {
                    Transformer::LeakyTank.apply_transformer(model);
                }
            }
            BoolFlag::PowerRegen => {
                if enabled {
                    Transformer::PowerRegen.apply_transformer(model);
                }
            }
            _ => {} // Not all BoolFlags correspond to transformers - skip 'em
        }
    }
}
