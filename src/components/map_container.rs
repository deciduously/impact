use components::control_container::ControlContainer;
use types::tiles::{Tile, Tiles};
use yew::prelude::*;
use yew::services::console::ConsoleService;

type ImpactMsg = super::super::Msg;

pub struct MapContainer {
    title: String,
    tiles: Tiles,
    onsignal: Option<Callback<ImpactMsg>>,
}

pub enum Msg {
    ButtonPressed(ImpactMsg),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub tiles: Tiles,
    pub onsignal: Option<Callback<ImpactMsg>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            tiles: Tiles::new(),
            onsignal: None,
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
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::ButtonPressed(msg) => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(msg);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.tiles = props.tiles;
        self.onsignal = props.onsignal;
        true
    }
}

impl<CTX> Renderable<CTX, MapContainer> for MapContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        let view_tile = |(_id, tile): (&u32, &Tile)| {
            html! {
                <div class="tile-title",>{&format!("{}", tile)}</div>
                <div class="tile-art",>{&tile.art}</div>
                <div class="tile-control",>
                <ControlContainer: buttons=&tile.buttons, onsignal=Msg::ButtonPressed,/>
                </div>
            }
        };
        html! {
            <div class=("container", "container-map"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    { for self.tiles.iter().map(view_tile) }
                </div>
            </div>
        }
    }
}
