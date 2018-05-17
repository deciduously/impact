use super::super::Model;
use types::{flags::FloatFlag, resources::Resource};

//hmm - redundant? I've gone astray from the Overbots way
// he's got transofrmers that collect multiple effects, one of which in LeakyTank
// the FloatFlag
pub enum Transformer {
    OxygenDelta(f64),
}

pub fn apply_transformers(model: &mut Model) {
    // iterate over FloatFlags - all transformers will have a corresponding FloatFlag
    for (f, amt) in &model.float_flags {
        match f {
            // Having trouble reusing Action.perform()
            FloatFlag::OxygenDepletion => {
                let r = model.resource_values.entry(Resource::Oxygen).or_insert(0.0);
                *r += amt;
            }
        }
    }
}
