use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::foundations::align_text::AlignText;
use crate::foundations::container::BBContainer;
use crate::modules::hero::BBHero;
use yew::prelude::*;

#[function_component(Hero)]
pub fn component() -> Html {
    html! {
            <BBHero
                title="Brooks Builds Learners"
                text="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor!"
            />
    }
}
