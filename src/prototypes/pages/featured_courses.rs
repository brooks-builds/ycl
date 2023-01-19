use crate::elements::icon::BBIconType;
use crate::elements::title::BBTitleLevel;
use crate::foundations::container::{BBContainer, BBContainerMargin};
use crate::foundations::tags::Tags;
use crate::{
    modules::card_list::{BBCardData, BBCardDataBuilder, BBCardList},
    prototypes::router::Route,
};
use gloo::console::log;
use yew::prelude::*;

#[function_component(FeaturedCourses)]
pub fn component() -> Html {
    let card_data = vec![BBCardDataBuilder::new()
        .title("Node JS")
        .text("Learn how to build web api's and server side code using JavaScript on the backend.")
        .tag(Tags::NodeJS)
        .link(Route::Home)
        .build(),
    BBCardDataBuilder::new()
        .title("Rust")
        .text("Learn how to program in Rust, all you need is some programming experience in any language.")
        .tag(Tags::Rust)
        .link(Route::Home)
        .build(),
    BBCardDataBuilder::new()
        .title("Yew")
        .text("Learn how to build frontend applications using Rust and web Assembly.")
        .tag(Tags::Yew)
        .link(Route::Home)
        .build(),
    ];

    let on_action = Callback::from(|_event: ()| {
        log!("The show all action button was pressed");
    });

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBCardList<Route>
                {card_data}
                card_title_level={BBTitleLevel::Two}
                icon={BBIconType::Star}
                title="Featured Courses"
                more={true}
                {on_action}
            />
        </BBContainer>
    }
}
