extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod types;

use components::{control_container::ControlContainer, resource_container::ResourceContainer};
use std::collections::HashMap;
use types::Resource;
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Model {
    tick: u64,
    resource_values: HashMap<Resource, u32>,
}

pub enum Msg {
    TickForward,
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
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::TickForward => {
                self.tick += 1;
                env.as_mut().log("increment time");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, env);
                env.as_mut().log("Bulk action");
            },
        }
        true
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
                    <ControlContainer: />
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
