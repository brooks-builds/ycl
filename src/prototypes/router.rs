use super::pages::{home::Home, site_header::SiteHeader, hero::Hero, featured_courses::FeaturedCourses};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/site_header")]
    SiteHeader,
    #[at("/hero")]
    Hero,
    #[at("/featured_courses")]
    FeaturedCourses,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::SiteHeader => html! { <SiteHeader /> },
        Route::Hero => html! { <Hero /> },
        Route::FeaturedCourses => html! { <FeaturedCourses /> },
    }
}
