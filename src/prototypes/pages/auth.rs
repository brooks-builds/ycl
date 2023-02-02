use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
    modules::auth_prompt::BBAuthPrompt,
};

#[function_component(Auth)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Auth"}</BBTitle>

            <BBTitle level={BBTitleLevel::Two}>{"Register Account Prompt"}</BBTitle>
            <BBAuthPrompt
                title_level={BBTitleLevel::Three}
                register={true}
                discord="http://twitch.tv/brookzerker"
                twitch="http://twitch.tv/brookzerker"
                youtube="http://twitch.tv/brookzerker"
            />
        </BBContainer>
    }
}
