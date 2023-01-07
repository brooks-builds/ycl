use ::yew::prelude::*;
use stylist::{css, yew::styled_component, Style};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon_type: BBIconType,
    #[prop_or_else(|| BBIconSize::Normal)]
    pub size: BBIconSize,
    #[prop_or_default]
    pub classes: Classes,
}

#[styled_component(BBIcon)]
pub fn component(props: &Props) -> Html {
    html! {
        <img
            src={props.icon_type.src()}
            alt={props.icon_type.alt()}
            class={classes!("img-fluid", "img-thumbnail", props.icon_type.css(&props.size), props.classes.clone())} />
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBIconType {
    Brand,
    Star,
    Heart,
    Contact,
    Mark,
    Twitter,
    Twitch,
    YouTubeSmall,
    Check,
}

impl BBIconType {
    pub fn src(&self) -> &'static str {
        match self {
            BBIconType::Brand => "/logo-bb-blue.svg",
            BBIconType::Star => "/star.svg",
            BBIconType::Heart => "/heart.svg",
            BBIconType::Contact => "/chat_bubble.svg",
            BBIconType::Mark => "/bb-logo-icon-blue.svg",
            BBIconType::Twitter => "/twitter.svg",
            BBIconType::Twitch => "/TwitchGlitchPurple.svg",
            BBIconType::YouTubeSmall => "/youtube_social_squircle_red.png",
            BBIconType::Check => "/check.svg",
        }
    }

    pub fn alt(&self) -> &'static str {
        match self {
            BBIconType::Brand => "Brooks Builds Brand icon",
            BBIconType::Star => "Star logo",
            BBIconType::Heart => "Heart logo",
            BBIconType::Contact => "Chat Bubble icon",
            BBIconType::Mark => "Brooks Builds small brand icon",
            BBIconType::Twitter => "Twitter logo",
            BBIconType::Twitch => "Twitch logo",
            BBIconType::YouTubeSmall => "YouTube logo",
            BBIconType::Check => "Checkmark",
        }
    }

    pub fn css(&self, size: &BBIconSize) -> Classes {
        let background = match self {
            Self::Star => Some(
                Style::new(css!(
                    r#"
                        background-color: inherit; 
                        border: none;
                        padding-bottom: 1rem;
                    "#
                ))
                .unwrap(),
            ),
            Self::Heart
            | Self::Contact
            | Self::Twitter
            | Self::Mark
            | Self::Twitch
            | Self::YouTubeSmall
            | Self::Check => Some(
                Style::new(css!(
                    r#"
                        background-color: inherit; 
                        border: none;
                    "#
                ))
                .unwrap(),
            ),
            _ => None,
        };

        classes!(background, size.css())
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBIconSize {
    Small,
    Normal,
    Large,
    Tiny,
}

impl BBIconSize {
    pub fn css(&self) -> Style {
        let css = match self {
            BBIconSize::Small => css!("width: 50px;"),
            BBIconSize::Normal => css!("width: 150px;"),
            BBIconSize::Large => css!("width: 300px;"),
            BBIconSize::Tiny => css!("width: 25px;"),
        };

        Style::new(css).unwrap()
    }
}
