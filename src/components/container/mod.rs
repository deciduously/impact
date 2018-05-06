use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Container {
    title: String,
}

pub enum Msg {
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "DefaultContainerComponent".into(),
        }
    }
}

impl<CTX> Component<CTX> for Container
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Container { title: props.title }
    }

    fn update(&mut self, _msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }
}

impl<CTX> Renderable<CTX, Container> for Container
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="container",>
                <div class="title",>{&self.title}</div>
            </div>
        }
    }
}
