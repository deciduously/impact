use std::{fmt, collections::HashMap};
//use super::super::Model;
use types::{actions::Action, flags::BoolFlag, resources::Resource};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    Wait,
    ActivateOxygen,
}

impl Button {
    // each button produces an impact::Msg to be passed up to the model
    pub fn action(&self) -> Vec<Action> {
        match *self {
            Button::Wait => vec![Action::Noop],
            Button::ActivateOxygen => vec![
                Action::SetBoolFlag(BoolFlag::OxygenMonitor),
                Action::SetResourceValue(Resource::Oxygen, 100),
                Action::SetResourceValue(Resource::Power, 1),
                Action::SetBoolFlag(BoolFlag::LeakyTank),
                Action::AddMessage("Oxygen Monitor Up".to_string()),
                Action::AddMessage("Losing 1 Oxygen per second".to_string()),
                Action::AddMessage("Regenerating 2 power per second".to_string()),
                Action::DisableButton(Button::ActivateOxygen),
            ],
        }
    }
    //pub fn visible(&self, model: &Model) -> bool {
    //    match *self {
    //        Button::Wait => true,
    //        Button::ActivateOxygen => {
    //            !model.bool_flags.get(&BoolFlag::OxygenMonitor).unwrap_or(&false)
    //        }
    //    }
    //}
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Button::Wait => "Wait 1 Second",
            Button::ActivateOxygen => "Activate Oxygen",
        };
        write!(f, "{}", s)
    }
}

pub type Buttons = HashMap<Button, bool>;

//fn button_cost(b: Button) -> Option<(Resource, i32)> {
//    match b {
//   }
//}

// tiles will work similar but also pass up their ID along with the Action?
