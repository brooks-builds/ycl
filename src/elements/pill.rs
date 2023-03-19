use yew::prelude::*;

use crate::foundations::color::BBColor;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color: BBColor,
    pub children: Children,
}

#[function_component(BBPill)]
pub fn component(props: &Props) -> Html {
    html! {
        <span class={classes!("badge", "rounded-pill", props.color.bg_color())}>{props.children.clone()}</span>
    }
}
