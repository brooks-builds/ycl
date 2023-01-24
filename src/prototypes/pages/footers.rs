use yew::prelude::*;

use crate::foundations::container::BBContainer;
use crate::modules::site_footer::BBSiteFooter;
use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
    modules::nav::navbar_link::BBNavbarLinkBuilder,
    prototypes::router::Route,
};

#[function_component(Footers)]
pub fn component() -> Html {
    let left_links = vec![
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("Courses")
            .active()
            .build()
            .unwrap(),
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("Community")
            .build()
            .unwrap(),
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("Support")
            .build()
            .unwrap(),
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("Contact")
            .build()
            .unwrap(),
    ];
    let right_links = vec![
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("FAQ")
            .build()
            .unwrap(),
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("Terms")
            .build()
            .unwrap(),
        BBNavbarLinkBuilder::new()
            .to(Route::Home)
            .label("Privacy")
            .build()
            .unwrap(),
    ];

    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Footers"}</BBTitle>
            <BBSiteFooter<Route>
                {left_links}
                {right_links}
            />
        </BBContainer>
    }
}
