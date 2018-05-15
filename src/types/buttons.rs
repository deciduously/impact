use std::fmt;
use types::{self, actions::Action, flags::{BoolFlag, FloatFlag}, resources::Resource};

#[derive(Clone, PartialEq, Eq)]
pub enum Button {
    Wait,
    ActivateOxygen,
}

impl Button {
    // each button produces an impact::Msg to be passed up to the model
    pub fn action(&self) -> types::Msg {
        use types::Msg;
        match *self {
            Button::Wait => Msg::PerformAction(Action::Noop),
            Button::ActivateOxygen => Msg::Bulk(vec![
                Msg::PerformAction(Action::SetBoolFlag(BoolFlag::OxygenMonitor)),
                Msg::PerformAction(Action::SetResourceValue(Resource::Oxygen, 100)),
                Msg::PerformAction(Action::SetFloatFlag(FloatFlag::OxygenDepletion, -1)),
                Msg::PerformAction(Action::AddMessage("Oxygen Monitor Up".to_string())),
            ]),
        }
    }
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

//fn button_cost(b: Button) -> Option<(Resource, i32)> {
//    match b {
//   }
//}

// store them structs?  title/actions?
// and then pass a vector of the active ones to the ControlContainer to turn into buttons that emit callbacks

// tiles will work similar but also pass up their ID along with the Action?
