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
                Action::AddMessage("Oxygen Monitor Up".to_string()),
                Action::AddMessage("Losing 10 Oxygen per second - tank leaky".to_string()),
                Action::DisableButton(Button::ActivateOxygen, 0),
                Action::EnableButton(Button::OpenToolbox, 0),
                Action::EnableButton(Button::OpenDoor, 0),
            ],
            Button::OpenToolbox => vec![
                Action::AddMessage(
                    "You unceremoniously dump the toolbox contents all over the ship".to_string(),
                ),
                Action::EnableButton(Button::ApplyTape, 0),
                Action::EnableButton(Button::FiddleControls, 0),
                Action::DisableButton(Button::OpenToolbox, 0),
            ],
            Button::ApplyTape => vec![
                Action::ClearBoolFlag(BoolFlag::LeakyTank),
                Action::AddMessage("Leak stopped - for now.".to_string()),
                Action::DisableButton(Button::ApplyTape, 0),
            ],
            Button::FiddleControls => vec![
                Action::SetResourceValue(Resource::Power, 1),
                Action::SetBoolFlag(BoolFlag::PowerRegen),
                Action::AddMessage("You hear a loud bang from the bottom of the ship".to_string()),
                Action::AddMessage(
                    "Your fuel cells are on and recharging from your excess oxygen".to_string(),
                ),
                Action::DisableButton(Button::FiddleControls, 0),
            ],
            Button::OpenDoor => vec![
                Action::AddMessage("You push the airlock open and immediately DIE.".to_string()),
                Action::AddMessage("Just kidding - everything is fine.".to_string()),
                Action::SetResourceValue(Resource::Chutzpah, 50),
                Action::AddTile(1, Tile::new("Field".into(), ".......!!!!!.....".into())),
                Action::DisableButton(Button::OpenDoor, 0),
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

// tiles will work similar but also pass up their ID along with the Action?
