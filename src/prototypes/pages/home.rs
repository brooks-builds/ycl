use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::foundations::container::BBContainerMargin;
use crate::foundations::{align_text::AlignText, container::BBContainer};
use crate::modules::card_list::{BBCardDataBuilder, BBCardList};
use crate::prototypes::router::Route;
use yew::prelude::*;

#[function_component(Home)]
pub fn component() -> Html {
    let card_data = vec![
        BBCardDataBuilder::new().title("Site Header").link(Route::SiteHeader).build(),
        BBCardDataBuilder::new().title("Hero").link(Route::Hero).build(),
        BBCardDataBuilder::new().title("Featured Courses").link(Route::FeaturedCourses).build(),
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
