use yew::{classes, function_component, html, Children, Classes, Html, Properties};
use yew_router::{components::Link, Routable};

use super::button::BBButtonStyle;

#[derive(Properties, PartialEq)]
pub struct Props<R>
where
    R: Routable + 'static,
{
    pub to: R,
    #[prop_or_default]
    pub button: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| BBButtonStyle::PrimaryLight)]
    pub button_style: BBButtonStyle,
    pub classes: Option<Classes>,
}

#[function_component(BBInternalLink)]
pub fn component<R: Routable + 'static>(props: &Props<R>) -> Html {
    let is_button = if props.button {
        Some(classes!("btn", props.button_style.class()))
    } else {
        None
    };
    let class = classes!(is_button, props.classes.clone());

    html! {
        <Link<R> to={props.to.clone()} classes={class}>{props.children.clone()}</Link<R>>
    }
}
