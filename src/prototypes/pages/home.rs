use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::foundations::container::BBContainerMargin;
use crate::foundations::{align_text::AlignText, container::BBContainer};
use crate::modules::card_list::{BBCardDataBuilder, BBCardList};
use crate::prototypes::router::Route;
use yew::prelude::*;

#[function_component(Home)]
pub fn component() -> Html {
    let card_data = vec![
        BBCardDataBuilder::new()
            .title("Site Header")
            .link(Route::SiteHeader)
            .build(),
        BBCardDataBuilder::new()
            .title("Heros")
            .link(Route::Heros)
            .build(),
        BBCardDataBuilder::new()
            .title("Cards")
            .link(Route::Cards)
            .build(),
        BBCardDataBuilder::new()
            .title("Footers")
            .link(Route::Footers)
            .build(),
        BBCardDataBuilder::new()
            .title("Navigations")
            .link(Route::Navs)
            .build(),
        BBCardDataBuilder::new()
            .title("Content")
            .link(Route::Content)
            .build(),
    ];
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle
                level={BBTitleLevel::One}
                align={AlignText::Center}
            >
                {"Brooks Builds Yew Component Library Prototypes"}
            </BBTitle>
            <BBCardList<Route>
                {card_data}
                card_title_level={BBTitleLevel::Two}
            />
        </BBContainer>
    }
}
