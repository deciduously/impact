extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod types;

use components::{control_container::ControlContainer, messages_container::MessagesContainer,
                 resource_container::ResourceContainer};
use types::{Msg, Tick, messages::Message, resources::Resources};
use std::collections::HashMap;
use yew::{prelude::*, services::console::ConsoleService};

pub struct Model {
    pub tick: Tick,
    pub resource_values: Resources,
    pub messages: Vec<Message>,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            tick: 0,
            resource_values: HashMap::new(),
            messages: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::PerformAction(action) => {
                env.as_mut().log(&format!("action: {:?}", action));
                &action.perform(self);
                true
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
                    <p>{&format!("Tick: {}", self.tick)}</p>
                    <ResourceContainer: resources=&self.resource_values,/>
                    <ControlContainer: onsignal=|action| Msg::PerformAction(action),/>
                    <MessagesContainer: messages=&self.messages,/>
                </div>
            </div>
        }
    }
}
// Map/Tiles (where MOST of the app lives)
