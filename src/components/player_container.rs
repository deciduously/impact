use components::map_container::MapContainer;
use types::{player::Player, tiles::defined_tiles};
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

// type ImpactMsg = super::super::Msg;

pub struct PlayerContainer {
    player: Player,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub player: Player,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            player: Player::default(),
        }
    }
}

impl Component for PlayerContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PlayerContainer {
            player: props.player,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.player = props.player;
        true
    }
}

impl Renderable<PlayerContainer> for PlayerContainer {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=("container", "container-Player"),>
                <div class="title",>{&self.player.name}</div>
                <div class="scroller",>
                    { format!("Name: {}, Current tile: {}", self.player.name, self.player.current_tile) }
                </div>
                <MapContainer: tile=defined_tiles(self.player.current_tile).unwrap(),/>
            </div>
        }
    }
}
