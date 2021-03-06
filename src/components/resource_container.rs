use types::resources::{Resource, Resources};
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct ResourceContainer {
    title: String,
    resources: Resources,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub resources: Resources,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            resources: Resources::new(),
        }
    }
}

impl Component for ResourceContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ResourceContainer {
            title: "Resources".to_string(),
            resources: props.resources,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.resources = props.resources;
        true
    }
}

impl Renderable<ResourceContainer> for ResourceContainer {
    // TODO individual Resources-ids and inner/chidren ids after scroller
    fn view(&self) -> Html<Self> {
        let view_resource = |(resource, (amt, delta)): (&Resource, &(i64, i64))| {
            // TODO resource-delta
            html! {
                <div class="resource",>
                    <span class="resource-title",>{&format!("{:?}", resource)}</span>
                    <span class="resource-amt",>{amt}</span>
                    <span class="resource-delta",>{&format!("{}/sec", delta)}</span>
                </div>
            }
        };
        html! {
            <div class=("container", "container-resources"),>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>{ for self.resources.iter().map(view_resource) }</div>
            </div>
        }
    }
}
