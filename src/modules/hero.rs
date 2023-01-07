use yew::{prelude::*, virtual_dom::VNode};

use crate::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        container::BBContainer,
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
}

#[function_component(BBHero)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer classes="px-4 my-5 bg-secondary py-4">
            <BBRow>
                {
                    props.media.render()
                }
                <BBCol width={props.media.main_width()} classes="my-auto">
                    <div>
                        <BBTitle level={props.title_level} align={AlignText::Center}>{props.title.clone()}</BBTitle>
                        <BBText align={AlignText::Center}>{props.text.clone()}</BBText>
                        {props.main.clone()}
                    </div>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}

#[derive(PartialEq, Clone)]
pub enum BBHeroLeftMedia {
    None,
    Image(VNode),
}

impl BBHeroLeftMedia {
    pub fn render(&self) -> Html {
        match self {
            BBHeroLeftMedia::None => html! {},
            BBHeroLeftMedia::Image(image) => html! {
                <BBCol width={BBColWidth::Six} classes="py-1">
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
            BBHeroLeftMedia::Image(_) => BBColWidth::Six,
        }
    }

    pub fn media_width(&self) -> Option<BBColWidth> {
        match self {
            BBHeroLeftMedia::None => None,
            _ => Some(BBColWidth::Six),
        }
    }
}
