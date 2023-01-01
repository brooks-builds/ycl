use gloo::console::log;
use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
    modules::card_list::{BBCardData, BBCardList},
    prototypes::router::Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PFeatures)]
pub fn component(_props: &Props) -> Html {
    let some_card_data = vec![
        BBCardData::<Route> {
            link: None,
            text: Some("first".to_owned()),
            title: "first card".to_owned(),
        },
        BBCardData::<Route> {
            link: None,
            text: Some("second".to_owned()),
            title: "second card".to_owned(),
        },
        BBCardData::<Route> {
            link: None,
            text: Some("third".to_owned()),
            title: "third card".to_owned(),
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
                more={true} />
            <BBCardList<Route>
                card_data={some_card_data}
                card_title_level={BBTitleLevel::Three}
                title="Cards but this is it"
                title_level={BBTitleLevel::Two}
                {on_action} />
        </main>
    }
}
