use crate::elements::title::BBTitleLevel;
use crate::foundations::tags::Tags;
use crate::{
    modules::card_list::{BBCardData, BBCardDataBuilder, BBCardList},
    prototypes::router::Route,
};
use yew::prelude::*;

#[function_component(FeaturedCourses)]
pub fn component() -> Html {
    let card_data = vec![BBCardDataBuilder::new()
        .title("Node JS")
        .text("Learn how to build web api's and server side code using JavaScript on the backend.")
        .tag(Tags::NodeJS)
        .build()];

    html! {
        <BBCardList<Route>
            {card_data}
            card_title_level={BBTitleLevel::Two}
        />
    }
}
