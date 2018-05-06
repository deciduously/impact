use types::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;

// NOTE TO SELF - there shouldn't be too many buttons here.
// Most will be tile-specific

pub struct ControlContainer {
    title: String,
    onsignal: Option<Callback<Action>>,
}

pub enum Msg {
    EndTurn,
    AddOxygen,
    AddTestMessage,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onsignal: Option<Callback<Action>>,
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
            Msg::EndTurn => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(Action::EndTurn);
                }
            }
            Msg::AddOxygen => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(Action::AddResourceValue(Resource::Oxygen, 1));
                }
            }
            Msg::AddTestMessage => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(Action::AddMessage("A Test Message".to_string()));
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
                    <button onclick=|_| Msg::EndTurn,>{"End Turn"}</button>
                    <button onclick=|_| Msg::AddOxygen,>{" Add Oxygen" }</button>
                    <button onclick=|_| Msg::AddTestMessage,>{" Try a Message "}</button>
                </div>
            </div>
        }
    }
}
