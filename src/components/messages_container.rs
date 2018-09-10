use types::messages::Message;
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

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

impl Component for MessagesContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        MessagesContainer {
            title: "Messages".into(),
            messages: props.messages,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.messages = props.messages;
        true
    }
}

impl Renderable<MessagesContainer> for MessagesContainer {
    fn view(&self) -> Html<Self> {
        let view_message = |message: &Message| {
            html! {
                <li>
                    <span class="message-time",>
                        {&format!("{}", message.time)}
                    </span>
                    {" "}{message.content.to_string()}
                </li>
            }
        };
        html! {
            <div class=("container", "container-messages"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>
                    <ul>{ for self.messages.iter().rev().map(view_message) }</ul>
                </div>
            </div>
        }
    }
}
