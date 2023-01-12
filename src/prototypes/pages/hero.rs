use yew::prelude::*;
use crate::foundations::align_text::AlignText;
use crate::foundations::container::BBContainer;
use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::modules::hero::BBHero;

#[function_component(Hero)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                {"Hero"}
            </BBTitle>
            <BBHero 
                title="Brooks Builds Learners"
                text="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor!"
            />
        </BBContainer>
    }
}
