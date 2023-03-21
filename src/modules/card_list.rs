use crate::foundations::tags::BBTag;
use crate::{
    elements::{icon::BBIconType, title::BBTitleLevel},
    foundations::{container::BBContainer, row::BBRow},
    modules::{card::BBCard, section_header::BBSectionHeader},
};
use gloo::console::log;
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
}

#[function_component(BBCardList)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let title_level = props.card_title_level.clone();

    let on_action = {
        let on_action = props.on_action.clone();
        Callback::from(move |_event| {
            on_action.emit(());
        })
    };

    html! {
        <BBContainer>
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
            <BBRow classes="d-flex justify-content-evenly">
                {
                    props.card_data.clone().into_iter().map(move |card_data| {
                        let onclick = {
                            let cb = card_data.onclick.unwrap_or_default();
                            let debug = props.debug;
                            let debug_name = props.debug_name.clone();

                            Callback::from(move |_: ()| {
                                if debug {
                                    log!(format!("Card in card list clicked for {debug_name}"));
                                }

                                cb.emit(());
                            })
                        };

                        let classes = classes!(card_data.tag.map(|tag| tag_class(tag)));

                        html! {
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
                            />
                        }
                    })
                    .collect::<Html>()
                }
            </BBRow>
        </BBContainer>
    }
}

#[derive(PartialEq, Clone)]
pub struct BBCardData<T>
where
    T: Routable + 'static,
{
    pub title: String,
    pub text: Option<String>,
    pub link: Option<T>,
    pub onclick: Option<Callback<()>>,
    pub card_type: BBCardType,
    pub tag: Option<BBTag>,
    pub href: Option<AttrValue>,
    pub href_text: AttrValue,
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

#[derive(PartialEq, Clone)]
pub struct BBCardDataBuilder<T>
where
    T: Routable + 'static,
{
    pub title: String,
    pub text: Option<String>,
    pub link: Option<T>,
    pub onclick: Option<Callback<()>>,
    pub card_type: BBCardType,
    pub tags: Option<BBTag>,
    pub href: Option<AttrValue>,
    pub href_text: AttrValue,
}

impl<T: Routable> BBCardDataBuilder<T> {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            text: None,
            link: None,
            onclick: None,
            card_type: BBCardType::Simple,
            tags: None,
            href: None,
            href_text: "".into(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
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
        }
    }
}
