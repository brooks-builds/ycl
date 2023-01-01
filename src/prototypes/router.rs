use super::pages::{features::PFeatures, headers::PHeaders, heroes::PHeros, home::PHome};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/headers")]
    Headers,
    #[at("/heroes")]
    Heroes,
    #[at("/features")]
    Features,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <PHome /> },
        Route::Headers => html! { <PHeaders /> },
        Route::Heroes => html! { <PHeros /> },
        Route::Features => html! { <PFeatures /> },
    }
}
