use types::tiles::{Tile};
use yew::prelude::{Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

type ImpactMsg = super::super::Msg;

pub struct MapContainer {
    title: String,
    tile: Tile,
    onsignal: Option<Callback<ImpactMsg>>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub tile: Tile,
    pub onsignal: Option<Callback<ImpactMsg>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            tile: Tile::default(),
            onsignal: None,
        }
    }
}

impl Component for MapContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        MapContainer {
            title: "Map".into(),
            tile: props.tile,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.tile = props.tile;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<MapContainer> for MapContainer {
    fn view(&self) -> Html<Self> {
        let view_tile = |tile: &Tile| {
            // Tiles arent going to have controls of their own
            // Clean this out before doing anything else.
            html! {
                <div class="tile-title",>{&format!("{}", tile)}</div>
                <div class="tile-art",>{&tile.art}</div>
            }
        };
        html! {
            <div class=("container", "container-map"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    { view_tile(&self.tile) }
                </div>
            </div>
        }
    }
}
