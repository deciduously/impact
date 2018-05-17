use types::{actions::Action, buttons::Button};
use yew::prelude::*;
use yew::services::console::ConsoleService;

type ImpactMsg = super::super::Msg;

// NOTE TO SELF - there shouldn't be too many buttons here.
// Most will be tile-specific

pub struct ControlContainer {
    title: String,
    buttons: Vec<Button>,
    onsignal: Option<Callback<ImpactMsg>>,
}

pub enum Msg {
    ButtonPressed(ImpactMsg),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub buttons: Vec<Button>,
    pub onsignal: Option<Callback<ImpactMsg>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            buttons: Vec::new(),
            onsignal: None,
        }
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
            buttons: props.buttons,
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
        self.buttons = props.buttons;
        self.onsignal = props.onsignal;
        true
    }
}

impl<CTX> Renderable<CTX, ControlContainer> for ControlContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        // YOu're having a lifetime problem - how to pass the message back up
        // this is a standin
        // ALSO why does ImpactMsg not work in here?
        let view_button = |button: &Button| {
            html! {
                <button onclick=|_| Msg::ButtonPressed(super::super::Msg::PerformAction(Action::Noop)),>{&format!("{}", button)}</button>
            }
        };
        html! {
            <div class="container",>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    { for self.buttons.iter().map(view_button) }
                </div>
            </div>
        }
    }
}
