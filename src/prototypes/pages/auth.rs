use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        align_text::AlignText,
        column::BBCol,
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
    },
    modules::{auth_prompt::BBAuthPrompt, icons_row::BBIconsRowList},
};

#[function_component(Auth)]
pub fn component() -> Html {
    let auth_icons = vec![
        BBIconsRowList::new(
            crate::elements::icon::BBIconType::Discord,
            "http://twitch.tv/brookzerker",
        ),
        BBIconsRowList::new(
            crate::elements::icon::BBIconType::Twitch,
            "http://twitch.tv/brookzerker",
        ),
        BBIconsRowList::new(
            crate::elements::icon::BBIconType::YouTubeSmall,
            "http://twitch.tv/brookzerker",
        ),
    ];

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBRow>
                <BBCol>
                    <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Auth"}</BBTitle>
                </BBCol>
            </BBRow>

            <BBRow>
                <BBCol>
                    <BBTitle level={BBTitleLevel::Two}>{"Register Account Prompt"}</BBTitle>
                </BBCol>
            </BBRow>

            <BBAuthPrompt
                title_level={BBTitleLevel::Three}
                register={true}
                {auth_icons}
            />
        </BBContainer>
    }
}
