extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod types;

use components::{map_container::MapContainer, messages_container::MessagesContainer,
                 resource_container::ResourceContainer};
use types::{actions::{apply_timeactions, Action}, buttons::Button, flags::BoolFlags,
            messages::Message, resources::Resources, tiles::{Tile, Tiles}, time::Time,
            transformers::apply_transformers};
use yew::{prelude::*, services::console::ConsoleService};

pub struct Model {
    time: Time,
    resource_values: Resources,
    messages: Vec<Message>,
    bool_flags: BoolFlags,
    //int_flags: IntFlags,
    //float_flags: FloatFlags,
    tiles: Tiles,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Tick,
    PerformAction(Action),
    Bulk(Vec<Msg>),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        let mut ret = Model {
            time: Time::new(),
            resource_values: Resources::new(),
            messages: Vec::new(),
            bool_flags: BoolFlags::new(),
            //int_flags: IntFlags::new(),
            //float_flags: FloatFlags::new(),
            tiles: Tiles::new(),
        };
        Action::AddTile(0, Tile::new("Ship".into(), ".::^::.".into())).perform(&mut ret);
        Action::EnableButton(0, Button::Wait).perform(&mut ret);
        ret
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Tick => {
                env.as_mut().log("tick");
                self.time.increment();
                apply_transformers(self);
                apply_timeactions(self);
                true
            }
            Msg::PerformAction(action) => {
                env.as_mut().log(&format!("action: {:?}", action));
                action.perform(self);
                self.update(Msg::Tick, env); // TODO - THIS IS A CORE MECHANIC - IS THIS REALLY EACH ACTION
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
                <span class="time",>{&format!("Time: {}", self.time)}</span>
                    <ResourceContainer: resources=&self.resource_values,/>
                    <MapContainer: tiles=&self.tiles, onsignal=|msg| msg,/>
                </div>
                <MessagesContainer: messages=&self.messages,/>
            </div>
        }
    }
}
