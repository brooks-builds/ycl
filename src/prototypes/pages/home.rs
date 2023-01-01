use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        column::{BBCol, BBColWidth},
        row::BBRow,
    },
    modules::card_list::{BBCardList, CardData},
    prototypes::router::Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHome)]
pub fn component(_props: &Props) -> Html {
    let snippet_cards = vec![CardData::<Route> {
        text: Some("Headers available for Brooks Builds projects".to_owned()),
        title: "Headers".to_owned(),
        link: Some(Route::Headers),
    },
    CardData::<Route> {
        text: Some("Heroes and other major call to action sections available for Brooks Builds projects".to_owned()),
        title: "Heroes".to_owned(),
        link: None
    },
    CardData::<Route> {
        text: Some("Showing off what features are available for products".to_owned()),
        title: "Features".to_owned(),
        link: None
    },
    CardData::<Route> {
        text: Some("Common navigation patterns ideal for offcanvas or multi-column layouts.".to_owned()),
        title: "Sidebars".to_owned(),
        link: None
    },
    CardData::<Route> {
        text: Some("Finish every page strong with an awesome footer, big or small.".to_owned()),
        title: "Footers".to_owned(),
        link: None
    },
    CardData::<Route> {
        text: Some("Enhance your dropdowns with filters, icons, custom styles, and more.".to_owned()),
        title: "Dropdowns".to_owned(),
        link: None
    },
    CardData::<Route> {
        text: Some("Extend list groups with utilities and custom styles for any content.".to_owned()),
        title: "List groups".to_owned(),
        link: None
    },
    CardData::<Route> {
        text: Some("Transform modals to serve any purpose, from feature tours to dialogs.".to_owned()),
        title: "Modals".to_owned(),
        link: None
    }];

    html! {
        <div>
            <BBTitle level={BBTitleLevel::One}>{"Examples"}</BBTitle>
            <BBRow>
                <BBCol width={BBColWidth::Three}>
                    <BBTitle level={BBTitleLevel::Two}>{"Snippets"}</BBTitle>
                </BBCol>
                <BBCol>
                    <BBCardList<Route> card_data={snippet_cards} card_title_level={BBTitleLevel::Three} />
                </BBCol>
            </BBRow>
        </div>
    }
}
