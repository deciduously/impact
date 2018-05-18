use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct MapContainer {
    title: String,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}

impl<CTX> Component<CTX> for MapContainer
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        MapContainer {
            title: "Map".into(),
        }
    }

    fn update(&mut self, _msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }
}

impl<CTX> Renderable<CTX, MapContainer> for MapContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class=("container", "container-map"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                {"map!"}
                </div>
            </div>
        }
    }
}
