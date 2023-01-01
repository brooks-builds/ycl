use yew::prelude::*;
use yew_router::Routable;

use crate::{elements::title::BBTitleLevel, foundations::row::BBRow, modules::card::BBCard};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub card_data: Vec<CardData<T>>,
    pub card_title_level: BBTitleLevel,
}

#[function_component(BBCardList)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let title_level = props.card_title_level.clone();

    html! {
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
    }
}

#[derive(PartialEq, Clone)]
pub struct CardData<T>
where
    T: Routable + 'static,
{
    pub title: String,
    pub text: Option<String>,
    pub link: Option<T>,
}
