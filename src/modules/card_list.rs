use yew::prelude::*;
use yew_router::Routable;

use crate::{
    elements::{icon::BBIconType, title::BBTitleLevel},
    foundations::{container::BBContainer, row::BBRow},
    modules::{card::BBCard, section_header::BBSectionHeader},
};

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
        <BBContainer full_width={true}>
            {
                if props.title.is_some() {
                    Some(html! {
                        <BBSectionHeader
                           icon={BBIconType::Star}
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
            <BBRow>
                {
                    props.card_data.clone().into_iter().map(move |card_data| {
                        html! {
                            <BBCard<T>
                            {title_level}
                            title={card_data.title}
                            text={card_data.text}
                            internal_link={card_data.link} />
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
}
