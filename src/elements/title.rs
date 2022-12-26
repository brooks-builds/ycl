use ::yew::prelude::*;

use crate::foundations::align_text::AlignText;

#[derive(PartialEq)]
pub enum BBTitleLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub level: BBTitleLevel,
    pub children: Children,
    pub align: Option<AlignText>,
}

#[function_component(BBTitle)]
pub fn component(props: &Props) -> Html {
    let classes = classes!(props.align.clone().unwrap_or_default().class());
    match props.level {
        BBTitleLevel::One => html! { <h1 class={classes}>{props.children.clone()}</h1> },
        BBTitleLevel::Two => html! { <h2 class={classes}>{props.children.clone()}</h2> },
        BBTitleLevel::Three => html! { <h3 class={classes}>{props.children.clone()}</h3> },
        BBTitleLevel::Four => html! { <h4 class={classes}>{props.children.clone()}</h4> },
        BBTitleLevel::Five => html! { <h5 class={classes}>{props.children.clone()}</h5> },
        BBTitleLevel::Six => html! { <h6 class={classes}>{props.children.clone()}</h6> },
    }
}
