use std::collections::HashMap;
use types::resources::{Resource, Resources};
use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct ResourceContainer {
    title: String,
    resources: Resources,
}

pub enum Msg {
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub resources: Resources,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            resources: HashMap::new(),
        }
    }
}

impl<CTX> Component<CTX> for ResourceContainer
where
    CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        ResourceContainer {
            title: "Resources".to_string(),
            resources: props.resources,
        }
    }

    fn update(&mut self, _msg: Self::Msg, _env: &mut Env<CTX, Self>) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.resources = props.resources;
        true
    }
}

impl<CTX> Renderable<CTX, ResourceContainer> for ResourceContainer
where
    CTX: AsMut<ConsoleService> + 'static,
{
    // TODO individual Resources-ids and inner/chidren ids after scroller
    fn view(&self) -> Html<CTX, Self> {
        let view_resource = |resource: (&Resource, &f64)| {
            html! {
                <>
                    <span class="resource-title",>{&format!("{:?}", resource.0)}</span>
                    <span class="resource-amt",>{resource.1}</span>
                </>
            }
        };
        html! {
            <div class="container",>
                <div class="title",>{&self.title}</div>
                <div class="scroller",>{ for self.resources.iter().map(view_resource) }</div>
            </div>
        }
    }
}
