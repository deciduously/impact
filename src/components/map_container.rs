use types::tiles::{Tile, Tiles};
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct MapContainer {
    title: String,
    tiles: Vec<Tile>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub tiles: Tiles,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            tiles: Tiles::new(),
        }
    }
}

impl<CTX> Component<CTX> for MapContainer
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        MapContainer {
            title: "Map".into(),
            tiles: props.tiles,
        }
    }

    fn update(&mut self, _msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.tiles = props.tiles;
        true
    }
}

impl<CTX> Renderable<CTX, MapContainer> for MapContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        let view_tile = |tile: &Tile| {
            html!{
                <span class="tile-title",>{&tile.name}</span>
            }
        };
        html! {
            <div class=("container", "container-map"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    {for self.tiles.iter().map(view_tile) }
                </div>
            </div>
        }
    }
}
