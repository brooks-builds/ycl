use yew::prelude::*;

use crate::{
    elements::{
        icon::{BBIcon, BBIconSize, BBIconType},
        nav_link::BBNavLink,
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{align_text::AlignText, column::BBCol, container::BBContainer, row::BBRow},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub discord: Option<AttrValue>,
    pub twitch: Option<AttrValue>,
    pub youtube: Option<AttrValue>,
    pub username_password: Option<AttrValue>,
    #[prop_or_else(|| BBTitleLevel::One)]
    pub title_level: BBTitleLevel,
    #[prop_or_default]
    pub register: bool,
}

#[function_component(BBAuthPrompt)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer>
            <BBRow>
                <BBCol>
                    <BBTitle
                        level={props.title_level.clone()}
                        align={AlignText::Center}
                    >
                        {
                            if props.register {
                                "Register Account"
                            } else {
                                "Login"
                            }
                        }
                    </BBTitle>
                </BBCol>
            </BBRow>
            <BBRow>
                <BBCol>
                    <BBText align={AlignText::Center}>
                        {
                            if props.register {
                                "Create new account using social links or username/password"
                            } else {
                                "Login to your account"
                            }
                        }
                    </BBText>
                </BBCol>
            </BBRow>
            <BBRow>
                <BBCol classes="d-flex justify-content-center mx-3">
                    {
                        props.discord.clone().map(|href| icon_link(BBIconType::Discord, href))
                    }
                    {
                        props.twitch.clone().map(|href| icon_link(BBIconType::Twitch, href))
                    }
                    {
                        props.youtube.clone().map(|href| icon_link(BBIconType::YouTubeSmall, href))
                    }
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}

fn icon_link(icon_type: BBIconType, href: AttrValue) -> Html {
    html! {
        <a {href} class="mx-2">
            <BBIcon {icon_type} size={BBIconSize::Small} />
        </a>
    }
}
