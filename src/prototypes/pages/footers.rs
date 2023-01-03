use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
    modules::{nav_footer::BBNavFooter, navbar::BBNavbarLink, site_footer::BBSiteFooter},
    prototypes::router::Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PFooters)]
pub fn component(_props: &Props) -> Html {
    let nav_footer_links = vec![
        BBNavbarLink::<Route> {
            active: false,
            to: Route::Home,
            label: "Home",
        },
        BBNavbarLink::<Route> {
            active: true,
            to: Route::Footers,
            label: "Footers",
        },
    ];

    let nav_footer_links_right = vec![
        BBNavbarLink::<Route> {
            active: false,
            to: Route::Home,
            label: "FAQ",
        },
        BBNavbarLink::<Route> {
            active: false,
            to: Route::Footers,
            label: "Terms",
        },
        BBNavbarLink::<Route> {
            active: false,
            to: Route::Footers,
            label: "Privacy",
        },
    ];
    html! {
        <main>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Footers"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Main site footer"}</BBTitle>
            <BBSiteFooter />
            <BBTitle level={BBTitleLevel::Two}>{"Nav Footer"}</BBTitle>
            <BBNavFooter<Route> links={nav_footer_links} right_links={nav_footer_links_right} />
        </main>
    }
}
