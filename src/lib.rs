extern crate stdweb;
#[macro_use]
extern crate yew;

//use stdweb::web::Date;
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Model {
    tick: u64,
}

pub enum Msg {
    TickForward,
    Bulk(Vec<Msg>),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model { tick: 0 }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::TickForward => {
                self.tick += 1;
                env.as_mut().log("increment time");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, env);
                env.as_mut().log("Bulk action");
            },
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div class="impact",>
                <div class="header",>{"IMPACT"}</div>
                <div class="body",>
                    <p>{ &format!("Tick: {}", self.tick) }</p>
                    <div class="container",>
                        <div class="title",>{"Resources: "}</div>
                    </div>
                    <div class="container",>
                        <div class="title",>{"Actions: "}</div>
                    </div>
                    <div class="container",>
                        <div class="title",>{"Inventory: "}</div>
                    </div>
                    <div class="container",>
                        <div class="title",>{"Map: "}</div>
                    </div>
                    <div class="container",>
                        <div class="title",>{"Messages: "}</div>
                    </div>
                </div>
            </div>
        }
    }
}
