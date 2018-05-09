use types::{self, buttons::Button};
use yew::prelude::*;
use yew::services::console::ConsoleService;

// NOTE TO SELF - there shouldn't be too many buttons here.
// Most will be tile-specific

// Also, buttons and button-actions should all be stored in types::buttons

pub struct ControlContainer {
    title: String,
    onsignal: Option<Callback<types::Msg>>,
}

pub enum Msg {
    ButtonPressed(types::Msg),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onsignal: Option<Callback<types::Msg>>,
}

impl Default for Props {
    fn default() -> Self {
        Props { onsignal: None }
    }
}

impl<CTX> Component<CTX> for ControlContainer
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        ControlContainer {
            title: "Control".to_string(),
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            // TODO this could/should be a macro, or at least abstracted out better
            Msg::ButtonPressed(msg) => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(msg);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.onsignal = props.onsignal;
        true
    }
}

impl<CTX> Renderable<CTX, ControlContainer> for ControlContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="container",>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    <button onclick=|_| Msg::ButtonPressed(Button::Wait.action()),>
                        {Button::Wait.text()}
                    </button>
                    <button onclick=|_| Msg::ButtonPressed(Button::ActivateOxygen.action()),>
                        {Button::ActivateOxygen.text()}
                    </button>
                </div>
            </div>
        }
    }
}
