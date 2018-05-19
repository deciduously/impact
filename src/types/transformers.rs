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
            Transformer::LeakyTank => vec![Consume(Resource::Oxygen, 10.0)],
            Transformer::PowerRegen => vec![
                Generate(Resource::Power, 2.0),
                Consume(Resource::Oxygen, 1.0),
            ],
        }
    }
}

pub enum Transformation {
    Generate(Resource, f64),
    Consume(Resource, f64),
}

impl Transformation {
    pub fn apply_transformation(&self, model: &mut Model) {
        use self::Transformation::*;
        match self {
            Generate(resource, delta) => {
                Action::AddResourceValue(*resource, *delta as i32).perform(model);
            }
            Consume(resource, delta) => {
                Action::AddResourceValue(*resource, -(*delta as i32)).perform(model);
            }
        }
    }
}

pub fn apply_transformers(model: &mut Model) {
    let bfs = model.bool_flags.clone(); // GROSS

    for (f, enabled) in bfs {
        match f {
            // this doesn't need to be a match - you're doin the same thing for any Transformer
            BoolFlag::LeakyTank => {
                if enabled {
                    for eff in Transformer::LeakyTank.effects().iter() {
                        eff.apply_transformation(model);
                    }
                }
            }
            BoolFlag::PowerRegen => {
                if enabled {
                    for eff in Transformer::PowerRegen.effects().iter() {
                        eff.apply_transformation(model);
                    }
                }
            }
            _ => {} // Not all BoolFlags correspond to transformers - skip 'em
        }
    }
}
