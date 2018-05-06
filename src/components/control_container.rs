use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct ControlContainer {
    title: String,
}

pub enum Msg {
}

impl<CTX> Component<CTX> for ControlContainer
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create((): Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        ControlContainer {
            title: "Control".to_string(),
        }
    }

    fn update(&mut self, _msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }
}

impl<CTX> Renderable<CTX, ControlContainer> for ControlContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    // TODO individual Resources-ids and inner/chidren ids after scroller
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="container",>
                <div class="title",>{&self.title}</div>
            </div>
        }
    }
}
