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

#[function_component(Cards)]
pub fn component() -> Html {
    let featured_course_cards = vec![BBCardDataBuilder::new()
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

    let community_driven_cards = vec![BBCardDataBuilder::new()
        .title("Discord")
        .text("Join the community, get and provide help about Rust, programming, and our careers")
        .link(Route::Home)
        .build(),
    BBCardDataBuilder::new()
        .title("Open Office")
        .text("Live stream every Friday is open office time, bring your questions and problems with the course and we'll work on them together")
        .link(Route::Home)
        .build(),
    BBCardDataBuilder::new()
        .title("Shared Resources")
        .text("Shared and starter code for your projects after you complete the courses")
        .link(Route::Home)
        .build(),
    ];

    let learner_support_cards = vec![BBCardDataBuilder::new()
        .title("Interactive Lessons")
        .text("Check your understanding of the lessons while taking the course and get feedback on your submissions")
        .link(Route::Home)
        .build(),
    BBCardDataBuilder::new()
        .title("Mentorship")
        .text("Meet with Brooks and other community mentors once per week")
        .link(Route::Home)
        .build(),
    BBCardDataBuilder::new()
        .title("Help Desk")
        .text("Give and receive help with your course assignments using a community driven help tool")
        .link(Route::Home)
        .build(),
    ];


    let on_action = Callback::from(|_event: ()| {
        log!("The action button was pressed");
    });

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBCardList<Route>
                card_data={featured_course_cards}
                card_title_level={BBTitleLevel::Two}
                icon={BBIconType::Star}
                title="Featured Courses"
                more={true}
                on_action={on_action.clone()}
            />
            <BBCardList<Route>
                card_data={community_driven_cards}
                card_title_level={BBTitleLevel::Two}
                icon={BBIconType::Heart}
                title="Community Driven"
                more={true}
                on_action={on_action.clone()}
                action="Meet Community"
            />
            <BBCardList<Route>
                card_data={learner_support_cards}
                card_title_level={BBTitleLevel::Two}
                icon={BBIconType::Contact}
                title="Learner Support"
                more={true}
                on_action={on_action.clone()}
                action="Features"
            />
        </BBContainer>
    }
}
