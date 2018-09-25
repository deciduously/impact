use types::{
    actions::msg_from_actions,
    buttons::{Button, ButtonID, Buttons},
};
use yew::prelude::{Callback, Component, ComponentLink, Html, Renderable, ShouldRender};

type ImpactMsg = super::super::Msg;

pub struct ControlContainer {
    title: String,
    buttons: Buttons,
    onsignal: Option<Callback<ImpactMsg>>,
}

pub enum Msg {
    ButtonPressed(ImpactMsg),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub buttons: Buttons,
    pub onsignal: Option<Callback<ImpactMsg>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            buttons: Buttons::new(),
            onsignal: None,
        }
    }
}

impl Component for ControlContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ControlContainer {
            title: "Control".to_string(),
            buttons: props.buttons,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonPressed(msg) => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(msg);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.buttons = props.buttons;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<ControlContainer> for ControlContainer {
    fn view(&self) -> Html<Self> {
        let view_button = |bid: &ButtonID| {
            let button = Button::by_index(*bid).unwrap();
            let m = msg_from_actions(&button.action());
            html! {
                <span class="control-button",>
                    <button onclick=|_| Msg::ButtonPressed(m.clone()),>{&format!("{}", button)}</button>
                </span>
            }
        };
        // issue 121 workaround for multiple classes
        html! {
            <div class=("container", "container-control"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    { for self.buttons.iter().map(view_button) }
                </div>
            </div>
        }
    }
}
