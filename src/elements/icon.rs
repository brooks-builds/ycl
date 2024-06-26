#![allow(non_camel_case_types)]
use ::yew::prelude::*;
use stylist::{css, yew::styled_component, Style};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon_type: BBIconType,
    #[prop_or_else(|| BBIconSize::Normal)]
    pub size: BBIconSize,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| true)]
    pub outline: bool,
}

#[styled_component(BBIcon)]
pub fn component(props: &Props) -> Html {
    let class = classes!(
        "img-fluid",
        if props.outline {
            Some("img-thumbnail")
        } else {
            None
        },
        props.icon_type.css(&props.size),
        props.classes.clone()
    );
    html! {
        <img
            src={props.icon_type.src()}
            alt={props.icon_type.alt()}
            {class} />
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Debug, Default)]
pub enum BBIconType {
    Brand,
    #[default]
    Star,
    Heart,
    Contact,
    Mark,
    Twitter,
    Twitch,
    YouTubeSmall,
    Check,
    Discord,
    Warning,
    Clear,
    Open,
}

impl BBIconType {
    pub fn src(&self) -> &'static str {
        match self {
            BBIconType::Brand => "/images/logo-bb-blue.svg",
            BBIconType::Star => "/images/star.svg",
            BBIconType::Heart => "/images/heart.svg",
            BBIconType::Contact => "/images/chat_bubble.svg",
            BBIconType::Mark => "/images/bb-logo-icon-blue.svg",
            BBIconType::Twitter => "/images/twitter.svg",
            BBIconType::Twitch => "/images/TwitchGlitchPurple.svg",
            BBIconType::YouTubeSmall => "/images/youtube_social_squircle_red.png",
            BBIconType::Check => "/images/check.svg",
            BBIconType::Discord => "/images/discord_logo.svg",
            BBIconType::Warning => "/images/warning_icon.svg",
            BBIconType::Clear => "/images/clear.svg",
            BBIconType::Open => "/images/chevron_right.svg",
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
            BBIconType::Discord => "Discord logo",
            BBIconType::Warning => "Warning logo",
            BBIconType::Clear => "Clear logo",
            BBIconType::Open => "",
        }
    }

    #[allow(non_upper_case_globals)]
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
            | Self::YouTubeSmall
            | Self::Check
            | Self::Discord
            | Self::Warning => Some(
                Style::new(css!(
                    r#"
                        background-color: inherit; 
                        border: none;
                    "#
                ))
                .unwrap(),
            ),
            Self::Twitch => Some(
                Style::new(css!(
                    r#"
                            background-color: inherit;
                            border: none;
                            position: relative;
                            transform: translateY(3px);
                        "#
                ))
                .unwrap(),
            ),
            BBIconType::Brand => None,
            BBIconType::Clear => None,
            BBIconType::Open => None,
        };

        classes!(background, size.css())
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBIconSize {
    Nano,
    Tiny,
    Smaller,
    Small,
    Normal,
    Large,
}

#[allow(non_upper_case_globals)]
impl BBIconSize {
    pub fn css(&self) -> Style {
        let css = match self {
            BBIconSize::Nano => css!("width: 15px;"),
            BBIconSize::Tiny => css!("width: 25px;"),
            BBIconSize::Smaller => css!("width: 35px;"),
            BBIconSize::Small => css!("width: 50px;"),
            BBIconSize::Normal => css!("width: 150px;"),
            BBIconSize::Large => css!("width: 300px;"),
        };

        Style::new(css).unwrap()
    }
}
