use std::{fmt, collections::HashMap};
use types::{actions::Action, flags::BoolFlag, resources::Resource, tiles::Tile};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Button {
    Wait,
    ActivateOxygen,
    OpenToolbox,
    ApplyTape,
    FiddleControls,
    OpenDoor,
}

impl Button {
    pub fn action(&self) -> Vec<Action> {
        match *self {
            Button::Wait => vec![Action::Noop],
            Button::ActivateOxygen => vec![
                Action::SetBoolFlag(BoolFlag::OxygenMonitor),
                Action::SetResourceValue(Resource::Oxygen, 1000),
                Action::SetBoolFlag(BoolFlag::LeakyTank),
                Action::AddMessage("Oxygen Monitor Up".into()),
                Action::AddMessage("Losing 10 Oxygen per second - tank leaky".into()),
                Action::DisableButton(0, Button::ActivateOxygen),
                Action::EnableButton(0, Button::OpenToolbox),
                Action::EnableButton(0, Button::OpenDoor),
            ],
            Button::OpenToolbox => vec![
                Action::AddMessage(
                    "You unceremoniously dump the toolbox contents all over the ship".into(),
                ),
                Action::EnableButton(0, Button::ApplyTape),
                Action::EnableButton(0, Button::FiddleControls),
                Action::DisableButton(0, Button::OpenToolbox),
            ],
            Button::ApplyTape => vec![
                Action::ClearBoolFlag(BoolFlag::LeakyTank),
                Action::AddMessage("Leak stopped - for now.".into()),
                Action::DisableButton(0, Button::ApplyTape),
            ],
            Button::FiddleControls => vec![
                Action::SetResourceValue(Resource::Power, 1),
                Action::SetBoolFlag(BoolFlag::PowerRegen),
                Action::AddMessage("You hear a loud bang from the bottom of the ship".into()),
                Action::AddMessage(
                    "Your fuel cells are on and recharging from your excess oxygen".into(),
                ),
                Action::DisableButton(0, Button::FiddleControls),
            ],
            Button::OpenDoor => vec![
                Action::AddMessage("You push the airlock open and immediately DIE.".into()),
                Action::AddMessage("Just kidding - everything is fine.".into()),
                Action::SetResourceValue(Resource::Chutzpah, 50),
                Action::AddTile(1, Tile::new("Field".into(), ".......!!!!!.....".into())),
                Action::DisableButton(0, Button::OpenDoor),
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
            Button::OpenDoor => "Open Airlock",
        };
        write!(f, "{}", s)
    }
}

pub type Buttons = HashMap<Button, bool>;

//fn button_cost(b: Button) -> Option<(Resource, i32)> {
//    match b {
//   }
//}
