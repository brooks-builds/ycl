use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Routable;

#[derive(Properties, PartialEq)]
pub struct Props<R>
where
    R: Routable + 'static,
{
    pub children: Children,
    pub to: Option<R>,
}

#[function_component(BBRouteOrNot)]
pub fn component<R: Routable + 'static>(props: &Props<R>) -> Html {
    match &props.to {
        Some(route) => html! {
            <Link<R> to={route.clone()}>{props.children.clone()}</Link<R>>
        },
        None => html! { <> { props.children.clone() } </> },
    }
}
