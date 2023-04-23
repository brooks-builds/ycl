use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::{BBContainer, BBContainerMargin},
        row::BBRow,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub text: AttrValue,
    #[prop_or_else(|| BBTitleLevel::One)]
    pub title_level: BBTitleLevel,
    #[prop_or_default]
    pub main: Children,
    #[prop_or_else(|| BBHeroLeftMedia::None)]
    pub media: BBHeroLeftMedia,
    pub subtitle: Option<AttrValue>,
}

#[function_component(BBHero)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer classes="bg-secondary py-4">
            <BBRow>
                {
                    props.media.render()
                }
                <BBCol width={props.media.main_width()} classes="my-auto">
                    <BBContainer margin={BBContainerMargin::Normal} classes={AlignText::Center.class()}>
                        <hgroup class={AlignText::Center.class()}>
                            <BBTitle level={props.title_level} align={AlignText::Center}>{props.title.clone()}</BBTitle>
                            {
                                props.subtitle.clone().map(|subtitle| html! { <p>{subtitle}</p> })
                            }
                        </hgroup>
                        <BBText align={AlignText::Center}>{props.text.clone()}</BBText>
                        {props.main.clone()}
                    </BBContainer>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}

#[derive(PartialEq, Clone)]
pub enum BBHeroLeftMedia {
    None,
    LeftMedia(VNode),
}

impl BBHeroLeftMedia {
    pub fn render(&self) -> Html {
        match self {
            BBHeroLeftMedia::None => html! {},
            BBHeroLeftMedia::LeftMedia(image) => html! {
                <BBCol width={BBColWidth::Six} classes={AlignText::Center.class()}>
                    {
                        image.clone()
                    }
                </BBCol>
            },
        }
    }

    pub fn main_width(&self) -> BBColWidth {
        match self {
            BBHeroLeftMedia::None => BBColWidth::None,
            BBHeroLeftMedia::LeftMedia(_) => BBColWidth::Six,
        }
    }

    pub fn media_width(&self) -> Option<BBColWidth> {
        match self {
            BBHeroLeftMedia::None => None,
            _ => Some(BBColWidth::Six),
        }
    }
}
