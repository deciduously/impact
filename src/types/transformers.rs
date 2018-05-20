use super::super::Model;
use types::{actions::Action, resources::Resource};

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

    fn apply_transformer(&self, model: &mut Model) {
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
    fn apply_transformation(&self, model: &mut Model) {
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
    let bfs = model.bool_flags.clone(); // stop cloning, Ben

    for (f, enabled) in bfs {
        if enabled {
            if let Some(tf) = f.transformer() {
                tf.apply_transformer(model);
            }
        }
    }
}
