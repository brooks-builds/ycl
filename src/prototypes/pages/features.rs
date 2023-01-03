use gloo::console::log;
use yew::prelude::*;

use crate::{
    elements::{
        icon::BBIconType,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::align_text::AlignText,
    modules::card_list::{BBCardData, BBCardList},
    prototypes::router::Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PFeatures)]
pub fn component(_props: &Props) -> Html {
    let onclick = Callback::from(|_event| {
        log!("call to action clicked");
    });

    let some_card_data = vec![
        BBCardData::<Route> {
            link: None,
            text: Some("first".to_owned()),
            title: "first card".to_owned(),
            onclick: None,
            card_type: crate::modules::card::BBCardType::Simple,
        },
        BBCardData::<Route> {
            link: None,
            text: Some("second".to_owned()),
            title: "second card".to_owned(),
            onclick: None,
            card_type: crate::modules::card::BBCardType::Simple,
        },
        BBCardData::<Route> {
            link: None,
            text: Some("third".to_owned()),
            title: "third card".to_owned(),
            onclick: None,
            card_type: crate::modules::card::BBCardType::Simple,
        },
        BBCardData::<Route> {
            link: None,
            text: None,
            title: "Request Course!".to_owned(),
            onclick: Some(onclick),
            card_type: crate::modules::card::BBCardType::CallToAction,
        },
    ];
    let on_action = Callback::from(|_event| {
        log!("Show all link clicked");
    });

    html! {
        <main>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Features"}</BBTitle>
            <BBCardList<Route>
                card_data={some_card_data.clone()}
                card_title_level={BBTitleLevel::Three}
                title="Cards but there are more"
                title_level={BBTitleLevel::Two}
                on_action={on_action.clone()}
                action="Show all"
                more={true}
                icon={BBIconType::Star} />
            <BBCardList<Route>
                card_data={some_card_data.clone()}
                card_title_level={BBTitleLevel::Three}
                title="Cards but this is it"
                title_level={BBTitleLevel::Two}
                on_action={on_action.clone()}
                icon={BBIconType::Heart} />
            <BBCardList<Route>
                card_data={some_card_data.clone()}
                card_title_level={BBTitleLevel::Three}
                title="This one has a chat icon"
                title_level={BBTitleLevel::Two}
                on_action={on_action.clone()}
                icon={BBIconType::Contact}
                debug={true}
                debug_name="last card list" />
        </main>
    }
}
