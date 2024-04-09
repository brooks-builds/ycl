#![allow(non_camel_case_types)]

use crate::foundations::column::BBCol;
use crate::foundations::container::BBContainerMargin;
use crate::foundations::row::BBRow;
use crate::foundations::tags::BBTag;
use crate::{
    elements::{icon::BBIconType, title::BBTitleLevel},
    foundations::container::BBContainer,
    modules::{card::BBCard, section_header::BBSectionHeader},
};
use gloo::console::info;
use yew::prelude::*;
use yew_router::Routable;

use super::card::BBCardType;

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub card_data: Vec<BBCardData<T>>,
    pub card_title_level: BBTitleLevel,
    pub title: Option<AttrValue>,
    #[prop_or_else(|| BBTitleLevel::Two)]
    pub title_level: BBTitleLevel,
    #[prop_or_else(|| AttrValue::from("Show All"))]
    pub action: AttrValue,
    #[prop_or_default]
    pub on_action: Callback<()>,
    #[prop_or_default]
    pub more: bool,
    pub icon: Option<BBIconType>,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub debug: bool,
    #[prop_or_default]
    pub debug_name: AttrValue,
    #[prop_or_default]
    pub wrap: bool,
    #[prop_or_default]
    pub test_id: AttrValue,
}

#[function_component(BBCardList)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let title_level = props.card_title_level;

    let on_action = {
        let on_action = props.on_action.clone();
        Callback::from(move |_event| {
            on_action.emit(());
        })
    };
    let section_class = classes!("d-flex", if props.wrap { Some("flex-wrap") } else { None });

    html! {
        <BBContainer margin={BBContainerMargin::None} test_id={props.test_id.clone()}>
            <BBRow>
                {
                    if props.title.is_some() {
                        Some(html! {
                            <BBSectionHeader
                               icon={props.icon}
                               title={props.title.clone().unwrap()}
                               title_level={BBTitleLevel::Two}
                               action={props.action.clone()}
                               more={props.more}
                               {on_action} />
                        })
                    } else {
                        None
                    }
                }
            </BBRow>
            <BBRow classes={section_class}>
                {
                    props.card_data.clone().into_iter().map(move |card_data| {
                        let onclick = {
                            let cb = card_data.onclick.unwrap_or_default();
                            let debug = props.debug;
                            let debug_name = props.debug_name.clone();

                            Callback::from(move |_: ()| {
                                if debug {
                                    info!(format!("Card in card list clicked for {debug_name}"));
                                }

                                cb.emit(());
                            })
                        };

                        let classes = classes!(card_data.tag.map(tag_class));

                        html! {
                            <BBCol classes="children-h-100">
                                <BBCard<T>
                                    {title_level}
                                    title={card_data.title}
                                    text={card_data.text}
                                    internal_link={card_data.link}
                                    card_type={card_data.card_type}
                                    {onclick}
                                    debug={props.debug}
                                    debug_name={props.debug_name.clone()}
                                    {classes}
                                    href={card_data.href}
                                    href_text={card_data.href_text}
                                    width={card_data.width}
                                />
                            </BBCol>
                        }
                    })
                    .collect::<Html>()
                }
            </BBRow>
        </BBContainer>
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BBCardData<T>
where
    T: Routable + 'static,
{
    pub title: AttrValue,
    pub text: Vec<AttrValue>,
    pub link: Option<T>,
    pub onclick: Option<Callback<()>>,
    pub card_type: BBCardType,
    pub tag: Option<BBTag>,
    pub href: Option<AttrValue>,
    pub href_text: AttrValue,
    pub width: BBCardDataWidth,
}

pub fn tag_class(tag: BBTag) -> &'static str {
    match tag {
        BBTag::NodeJS => "text-bg-node",
        BBTag::Axum => "text-bg-axum",
        BBTag::Yew => "text-bg-yew",
        BBTag::Rust => "text-bg-rust",
        BBTag::Unknown => "",
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BBCardDataBuilder<T>
where
    T: Routable + 'static,
{
    pub title: AttrValue,
    pub text: Vec<AttrValue>,
    pub link: Option<T>,
    pub onclick: Option<Callback<()>>,
    pub card_type: BBCardType,
    pub tags: Option<BBTag>,
    pub href: Option<AttrValue>,
    pub href_text: AttrValue,
    pub width: BBCardDataWidth,
}

impl<T: Routable> BBCardDataBuilder<T> {
    pub fn new() -> Self {
        Self {
            title: AttrValue::default(),
            text: vec![],
            link: None,
            onclick: None,
            card_type: BBCardType::Simple,
            tags: None,
            href: None,
            href_text: "".into(),
            width: BBCardDataWidth::Auto,
        }
    }

    pub fn title(mut self, title: impl Into<AttrValue>) -> Self {
        self.title = title.into();
        self
    }

    pub fn add_text(mut self, text: impl Into<AttrValue>) -> Self {
        self.text.push(text.into());
        self
    }

    pub fn link(mut self, link: T) -> Self {
        self.link = Some(link);
        self
    }

    pub fn onclick(mut self, onclick: Callback<()>) -> Self {
        self.onclick = Some(onclick);
        self
    }

    pub fn card_type(mut self, card_type: BBCardType) -> Self {
        self.card_type = card_type;
        self
    }

    pub fn tag(mut self, tag: BBTag) -> Self {
        self.tags = Some(tag);
        self
    }

    pub fn href(mut self, href: impl Into<AttrValue>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn href_text(mut self, href_text: impl Into<AttrValue>) -> Self {
        self.href_text = href_text.into();
        self
    }

    pub fn width(mut self, width: BBCardDataWidth) -> Self {
        self.width = width;
        self
    }

    pub fn build(self) -> BBCardData<T> {
        BBCardData {
            title: self.title,
            text: self.text,
            link: self.link,
            onclick: self.onclick,
            card_type: self.card_type,
            tag: self.tags,
            href: self.href,
            href_text: self.href_text,
            width: self.width,
        }
    }
}

impl<T: Routable> Default for BBCardDataBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub enum BBCardDataWidth {
    #[default]
    Auto,
    Small,
    Medium,
}
