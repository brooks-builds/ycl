use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{align_text::AlignText, container::BBContainer, row::BBRow},
    modules::hero::BBHero,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHeros)]
pub fn component(_props: &Props) -> Html {
    html! {
        <BBContainer full_width={true}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Heroes"}</BBTitle>
            <BBRow>
               <BBTitle level={BBTitleLevel::Two} align={AlignText::Left}>{"Billboard"}</BBTitle>
               <BBHero
                    title="Brooks Builds Learners!"
                    text="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor!" />
            </BBRow>
        </BBContainer>
    }
}
