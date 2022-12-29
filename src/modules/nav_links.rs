use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub position: BBNavLinkPosition,
}

#[function_component(BBNavLinks)]
pub fn component(props: &Props) -> Html {
    html! {
        <ul class={classes!("navbar-nav", props.position.class(), "mb-2", "mb-lg-0")}>
            {props.children.clone()}
        </ul>
    }
}

#[derive(PartialEq)]
pub enum BBNavLinkPosition {
    Center,
    Right,
}

impl BBNavLinkPosition {
    pub fn class(&self) -> &'static str {
        match self {
            BBNavLinkPosition::Center => "ms-auto me-auto",
            BBNavLinkPosition::Right => "ms-auto",
        }
    }
}
