use super::pages::{headers::PHeaders, home::PHome};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/headers")]
    Headers,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <PHome /> },
        Route::Headers => html! { <PHeaders /> },
    }
}
