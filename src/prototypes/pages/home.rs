use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{
        column::{BBCol, BBColWidth},
        row::BBRow,
    },
    modules::card_list::{BBCardList, CardData},
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHome)]
pub fn component(_props: &Props) -> Html {
    let snippet_cards = vec![CardData {
        text: Some("Headers available for Brooks Builds projects".to_owned()),
        title: "Headers".to_owned(),
    },
    CardData {
        text: Some("Heroes and other major call to action sections available for Brooks Builds projects".to_owned()),
        title: "Heroes".to_owned(),
    },
    CardData {
        text: Some("Showing off what features are available for products".to_owned()),
        title: "Features".to_owned(),
    },
    CardData {
        text: Some("Common navigation patterns ideal for offcanvas or multi-column layouts.".to_owned()),
        title: "Sidebars".to_owned(),
    },
    CardData {
        text: Some("Finish every page strong with an awesome footer, big or small.".to_owned()),
        title: "Footers".to_owned(),
    },
    CardData {
        text: Some("Enhance your dropdowns with filters, icons, custom styles, and more.".to_owned()),
        title: "Dropdowns".to_owned(),
    },
    CardData {
        text: Some("Extend list groups with utilities and custom styles for any content.".to_owned()),
        title: "List groups".to_owned(),
    },
    CardData {
        text: Some("Transform modals to serve any purpose, from feature tours to dialogs.".to_owned()),
        title: "Modals".to_owned(),
    }];

    html! {
        <div>
            <BBTitle level={BBTitleLevel::One}>{"Examples"}</BBTitle>
            <BBRow>
                <BBCol width={BBColWidth::Three}>
                    <BBTitle level={BBTitleLevel::Two}>{"Snippets"}</BBTitle>
                </BBCol>
                <BBCol>
                    <BBCardList card_data={snippet_cards} card_title_level={BBTitleLevel::Three} />
                </BBCol>
            </BBRow>
        </div>
    }
}
