use std::{fmt, collections::HashMap};
use types::{actions::Action, flags::BoolFlag, resources::Resource};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    Wait,
    ActivateOxygen,
    OpenToolbox,
    ApplyTape,
    FiddleControls,
}

impl Button {
    // each button produces an impact::Msg to be passed up to the model
    pub fn action(&self) -> Vec<Action> {
        match *self {
            Button::Wait => vec![Action::Noop],
            Button::ActivateOxygen => vec![
                Action::SetBoolFlag(BoolFlag::OxygenMonitor),
                Action::SetResourceValue(Resource::Oxygen, 1000),
                Action::SetBoolFlag(BoolFlag::LeakyTank),
                Action::AddMessage("Oxygen Monitor Up".to_string()),
                Action::AddMessage("Losing 10 Oxygen per second - tank leaky".to_string()),
                Action::DisableButton(Button::ActivateOxygen),
                Action::EnableButton(Button::OpenToolbox),
            ],
            Button::OpenToolbox => vec![
                Action::AddMessage(
                    "You unceremoniously dump the toolbox contents all over the ship".to_string(),
                ),
                Action::EnableButton(Button::ApplyTape),
                Action::EnableButton(Button::FiddleControls),
                Action::DisableButton(Button::OpenToolbox),
            ],
            Button::ApplyTape => vec![
                Action::ClearBoolFlag(BoolFlag::LeakyTank),
                Action::AddMessage("Leak stopped - for now.".to_string()),
                Action::DisableButton(Button::ApplyTape),
            ],
            Button::FiddleControls => vec![
                Action::SetResourceValue(Resource::Power, 1),
                Action::SetBoolFlag(BoolFlag::PowerRegen),
                Action::AddMessage("You hear a loud bang from the bottom of the ship".to_string()),
                Action::AddMessage(
                    "Your fuel cells are on and recharging from your excess oxygen".to_string(),
                ),
                Action::DisableButton(Button::FiddleControls),
            ],
        }
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Button::Wait => "Wait 1 Second",
            Button::ActivateOxygen => "Activate Oxygen",
            Button::OpenToolbox => "Search Toolbox",
            Button::ApplyTape => "Apply Scotch Tape to Tank",
            Button::FiddleControls => "Mess with the control panel",
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
