use crate::foundations::container::BBContainer;
use crate::modules::nav::navbar::BBNavbar;
use crate::modules::nav::navbar_link::BBNavbarLink;
use crate::prototypes::router::Route;
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

    html! {
        <BBContainer>
            <BBNavbar<Route>
                create_account_route={Route::Home}
                is_authenticated={false}
                links={navbar_links}
                login_route={Route::Home}
                show_brand={true}
            />
        </BBContainer>
    }
}
