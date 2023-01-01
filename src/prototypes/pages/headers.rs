use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    modules::navbar::{BBNavbar, BBNavbarLink},
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
            <BBNavbar<Route>
                links={navbar_links}
                is_authenticated={false}
                login_route={Route::Home}
                create_account_route={Route::Home} />
        </main>
    }
}
