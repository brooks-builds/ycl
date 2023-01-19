use super::pages::{
    featured_courses::FeaturedCourses, hero::Hero, home::Home, lms_promo::LmsPromo,
    site_header::SiteHeader, community_banner::CommunityBanner,
};
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
    #[at("/lms_promo")]
    LmsPromo,
    #[at("/community_banner")]
    CommunityBanner,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::SiteHeader => html! { <SiteHeader /> },
        Route::Hero => html! { <Hero /> },
        Route::FeaturedCourses => html! { <FeaturedCourses /> },
        Route::LmsPromo => html! { <LmsPromo /> },
        Route::CommunityBanner => html! { <CommunityBanner /> },
    }
}
