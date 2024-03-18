use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::foundations::align_text::AlignText;
use crate::foundations::container::BBContainer;
use crate::foundations::roles::BBRole;
use crate::modules::nav::navbar::BBNavbar;
use crate::modules::nav::navbar_link::BBNavbarLink;
use crate::prototypes::router::{Route, HOME_ROUTE};
use yew::prelude::*;

#[function_component(SiteHeader)]
pub fn component() -> Html {
    let navbar_links = vec![
        BBNavbarLink {
            active: true,
            label: "Site Headers",
            to: Route::Home,
        },
        BBNavbarLink {
            active: false,
            label: "Community",
            to: Route::Home,
        },
        BBNavbarLink {
            active: false,
            label: "Support",
            to: Route::Home,
        },
    ];

    let role = BBRole::Learner;

    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Navbars"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Navbar - logged out"}</BBTitle>
            <BBNavbar<Route>
                is_authenticated={false}
                links={navbar_links.clone()}
                show_brand={true}
            />
            <BBTitle level={BBTitleLevel::Two}>{"Navbar - logged in as admin"}</BBTitle>
            <BBNavbar<Route>
                is_authenticated={true}
                links={navbar_links}
                show_brand={true}
                username="Brooks"
                {role}
            />
        </BBContainer>
    }
}
