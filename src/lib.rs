extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod types;

use components::{control_container::ControlContainer, messages_container::MessagesContainer,
                 resource_container::ResourceContainer};
use std::collections::HashMap;
use types::{Msg, actions::{Action, TimeAction}, buttons::Button,
            flags::{BoolFlags, FloatFlags, IntFlags}, messages::Message, resources::Resources,
            time::Time};
use yew::{prelude::*, services::console::ConsoleService};

pub struct Model {
    time: Time,
    resource_values: Resources,
    messages: Vec<Message>,
    bool_flags: BoolFlags,
    int_flags: IntFlags,
    float_flags: FloatFlags,
    buttons: Vec<Button>,
    timeactions: Vec<TimeAction>,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            time: Time::new(),
            resource_values: HashMap::new(),
            messages: Vec::new(),
            bool_flags: HashMap::new(),
            int_flags: HashMap::new(),
            float_flags: HashMap::new(),
            buttons: vec![Button::Wait, Button::ActivateOxygen], // TODO placehlder - these need actions to insert/remove
            timeactions: vec![
                TimeAction::new(1, Action::AddMessage("It's been A SECOND".to_string())),
            ],
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Tick => {
                env.as_mut().log(&format!("tick"));
                self.time.increment();
                true
            }
            Msg::PerformAction(action) => {
                env.as_mut().log(&format!("action: {:?}", action));
                &action.perform(self);
                self.update(types::Msg::Tick, env);
                false
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg, env);
                    env.as_mut().log("Bulk action");
                }
                true
            }
        }
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="impact",>
                <div class="header",>{"IMPACT"}</div>
                <div class="body",>
                    <span class="time",>{&format!("Time: {}", self.time.clone())}</span>
                    <ResourceContainer: resources=&self.resource_values,/>
                    <ControlContainer: buttons=&self.buttons, onsignal=|msg| msg,/>
                    <MessagesContainer: messages=&self.messages,/>
                </div>
            </div>
        }
    }
}
// Map/Tiles (where MOST of the app lives)
