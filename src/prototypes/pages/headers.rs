use yew::prelude::*;

use crate::{
    elements::{
        icon::BBIconType,
        title::{BBTitle, BBTitleLevel},
    },
    modules::{
        navbar::{BBNavbar, BBNavbarLink},
        section_header::BBSectionHeader,
    },
    prototypes::router::Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHeaders)]
pub fn component(_props: &Props) -> Html {
    let navbar_links = vec![
        BBNavbarLink {
            label: "Home",
            to: Route::Home,
            active: false,
        },
        BBNavbarLink {
            label: "Headers",
            to: Route::Headers,
            active: true,
        },
    ];
    html! {
        <main>
            <BBTitle level={BBTitleLevel::One}>{"Headers"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Main Nav Header"}</BBTitle>
            <BBNavbar<Route>
                links={navbar_links}
                is_authenticated={false}
                login_route={Route::Home}
                create_account_route={Route::Home} />
            <BBTitle level={BBTitleLevel::Two}>{"Section Header with more to show"}</BBTitle>
            <BBSectionHeader
                title="Section Header"
                title_level={BBTitleLevel::Three}
                icon={BBIconType::Star}
                more={true} />
        </main>
    }
}
