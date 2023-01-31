use super::pages::{
    cards::Cards, content::Content, footers::Footers, heros::Heros, home::Home, navs::Navs,
    site_header::SiteHeader,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/site_header")]
    SiteHeader,
    #[at("/heros")]
    Heros,
    #[at("/cards")]
    Cards,
    #[at("/footers")]
    Footers,
    #[at("/navs")]
    Navs,
    #[at("/content")]
    Content,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::SiteHeader => html! { <SiteHeader /> },
        Route::Heros => html! { <Heros /> },
        Route::Cards => html! { <Cards /> },
        Route::Footers => html! { <Footers /> },
        Route::Navs => html! { <Navs /> },
        Route::Content => html! { <Content /> },
    }
}
