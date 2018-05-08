use types::{self, actions::Action, flags::{BoolFlag, FloatFlag}, resources::Resource};
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
    ActivateOxygen,
    Wait,
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
            Msg::ActivateOxygen => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(types::Msg::Bulk(vec![
                        types::Msg::PerformAction(Action::SetBoolFlag(BoolFlag::OxygenMonitor)),
                        types::Msg::PerformAction(Action::SetResourceValue(Resource::Oxygen, 100)),
                        types::Msg::PerformAction(Action::SetFloatFlag(
                            FloatFlag::OxygenDepletion,
                            -1,
                        )),
                        types::Msg::PerformAction(Action::AddMessage(
                            "Oxygen Monitor Up".to_string(),
                        )),
                    ]));
                }
            }
            Msg::Wait => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(types::Msg::PerformAction(Action::Noop));
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
                    <button onclick=|_| Msg::Wait,>{"Wait 1 second"}</button>
                    <button onclick=|_| Msg::ActivateOxygen,>{"Power Up Oxygen Meter"}</button>
                </div>
            </div>
        }
    }
}
