use yew::{classes, function_component, html, Callback, Children, Classes, Html, Properties};
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
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component(BBInternalLink)]
pub fn component<R: Routable + 'static>(props: &Props<R>) -> Html {
    let is_button = if props.button {
        Some(classes!("btn", props.button_style.class()))
    } else {
        None
    };
    let class = classes!(is_button, props.classes.clone());
    let props_onclick = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        props_onclick.emit(());
    });

    html! {
        <span {onclick}>
            <Link<R> to={props.to.clone()} classes={class} disabled={props.disabled}>{props.children.clone()}</Link<R>>
        </span>
    }
}
