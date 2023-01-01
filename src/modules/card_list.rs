use yew::prelude::*;

use crate::{elements::title::BBTitleLevel, foundations::row::BBRow, modules::card::BBCard};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub card_data: Vec<CardData>,
    pub card_title_level: BBTitleLevel,
}

#[function_component(BBCardList)]
pub fn component(props: &Props) -> Html {
    let title_level = props.card_title_level.clone();

    html! {
        <BBRow>
            {
                props.card_data.clone().into_iter().map(move |card_data| {
                    html! {
                        <BBCard {title_level} title={card_data.title} text={card_data.text} />
                    }
                })
                .collect::<Html>()
            }
        </BBRow>
    }
}

#[derive(PartialEq, Clone)]
pub struct CardData {
    pub title: String,
    pub text: Option<String>,
}
