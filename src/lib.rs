extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod types;

use components::{
    control_container::ControlContainer, messages_container::MessagesContainer,
    player_container::PlayerContainer, resource_container::ResourceContainer,
};
use types::{
    actions::{apply_timeactions, Action},
    buttons::Buttons,
    flags::BoolFlags,
    messages::Message,
    player::Player,
    resources::Resources,
    tiles::Tiles,
    time::Time,
    transformers::apply_transformers,
};
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    time: Time,
    resource_values: Resources,
    messages: Vec<Message>,
    bool_flags: BoolFlags,
    //int_flags: IntFlags,
    //float_flags: FloatFlags,
    tiles: Tiles,
    buttons: Buttons,
    player: Player,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Tick,
    PerformAction(Action),
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut ret = Model {
            time: Time::new(),
            resource_values: Resources::new(),
            messages: Vec::new(),
            bool_flags: BoolFlags::new(),
            //int_flags: IntFlags::new(),
            //float_flags: FloatFlags::new(),
            tiles: Tiles::new(),
            buttons: Buttons::new(),
            player: Player::new(),
        };
        Action::AddTile(0).perform(&mut ret);
        ret
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => {
                self.time.increment();
                apply_transformers(self);
                apply_timeactions(self);
                true
            }
            Msg::PerformAction(action) => {
                action.perform(self);
                self.update(Msg::Tick); // TODO - THIS IS A CORE MECHANIC - IS THIS REALLY EACH ACTION
                true
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                }
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="impact",>
                <div class="header",>{"IMPACT"}</div>
                <div class="body",>
                  <span class="time",>{&format!("Time: {}", self.time)}</span>
                   <ResourceContainer: resources=&self.resource_values,/>

                   <ControlContainer: buttons=&self.buttons, onsignal=|msg| msg,/>
                   <PlayerContainer: player=&self.player,/>
                </div>
                <MessagesContainer: messages=&self.messages,/>
            </div>
            <footer>
                    <a href="http://deciduously.com",>{"deciduously.com"}</a>{ " - " }<a href="https://github.com/deciduously/impact",>{"source"}</a>
            </footer>
        }
    }
}
