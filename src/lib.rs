extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod types;

use components::{control_container::ControlContainer, messages_container::MessagesContainer,
                 resource_container::ResourceContainer};
use std::collections::HashMap;
use types::{Tick, actions::Action, messages::Message, resources::Resources};
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Model {
    tick: Tick,
    resource_values: Resources,
    messages: Vec<Message>,
}

pub enum Msg {
    PerformAction(Action),
    Bulk(Vec<Msg>),
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
            // could this be moved into types/action as in impl
            Msg::PerformAction(action) => {
                use types::actions::*;
                match action {
                    Action::AddResourceValue(resource, delta) => {
                        // TODO add min/maxes, and check here
                        let r = self.resource_values.entry(resource).or_insert(0.0);
                        *r += delta as f64;
                        env.as_mut()
                            .log(&format!("adding {} {:?}", delta, resource));
                    }
                    Action::SetResourceValue(resource, amt) => {
                        self.resource_values.insert(resource, amt as f64);
                        env.as_mut()
                            .log(&format!("setting {:?} to {}", resource, amt));
                    }
                    Action::AddMessage(message) => {
                        self.messages.push(Message::new(message, self.tick));
                        env.as_mut().log("adding message");
                    }
                };
                self.tick += 1; // TODO Model::tick() which will apply transformers and then tick fwd
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
// Resources
// Actions (global, inner map tiles will have thier own buttons)
// Inventory
// Map (where MOST of the app lives)
// Messages
