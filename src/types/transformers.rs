use super::super::Model;
use types::{flags::BoolFlag, resources::Resource};

// Each transformer should have a corresponding BoolFlag
pub enum Transformer {
    LeakyTank,
}

impl Transformer {
    pub fn effects(&self) -> Vec<Transformation> {
        use self::Transformation::*;
        match self {
            Transformer::LeakyTank => vec![
                Consume(Resource::Oxygen, 1.0),
                Generate(Resource::Power, 2.0),
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
                // Add positive resource value
                // TODO make this a method on Resource, for use here and in Action
                let r = model.resource_values.entry(*resource).or_insert(0.0);
                *r += delta;
            }
            Consume(resource, delta) => {
                // Add negative resource value
                let r = model.resource_values.entry(*resource).or_insert(0.0);
                *r -= delta;
            }
        }
    }
}

pub fn apply_transformers(model: &mut Model) {
    let bfs = model.bool_flags.clone(); // GROSS

    for (f, enabled) in bfs {
        match f {
            BoolFlag::LeakyTank => {
                if enabled {
                    for eff in Transformer::LeakyTank.effects().iter() {
                        eff.apply_transformation(model);
                    }
                }
            }
            _ => {} // Not all BoolFlags correspond to transformers - skip 'em
        }
    }
}
