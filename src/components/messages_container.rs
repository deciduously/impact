use types::messages::Message;
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct MessagesContainer {
    title: String,
    messages: Vec<Message>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub messages: Vec<Message>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            messages: Vec::new(),
        }
    }
}

impl<CTX> Component<CTX> for MessagesContainer
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        MessagesContainer {
            title: "Messages".into(),
            messages: props.messages,
        }
    }

    fn update(&mut self, _msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.messages = props.messages;
        true
    }
}

impl<CTX> Renderable<CTX, MessagesContainer> for MessagesContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        let view_message = |message| {
            html! {
                <li>{&format!("{}", message)}</li>
            }
        };
        html! {
            <div class="container",>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    <ul>{ for self.messages.iter().map(view_message) }</ul>
                </div>
            </div>
        }
    }
}
