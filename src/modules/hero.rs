use yew::prelude::*;

use crate::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::align_text::AlignText,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub text: AttrValue,
}

#[function_component(BBHero)]
pub fn component(props: &Props) -> Html {
    html! {
        <div class="px-4 py-5 my-5 bg-secondary">
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{props.title.clone()}</BBTitle>
            <BBText align={AlignText::Center}>{props.text.clone()}</BBText>
        </div>
    }
}
