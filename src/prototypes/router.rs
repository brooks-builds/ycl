use super::pages::{
    auth::Auth, banners::Banners, cards::Cards, content::Content, dual_list::DualList,
    footers::Footers, forms::PForms, heros::Heros, home::Home, navs::Navs, site_header::SiteHeader,
    tables::TablesPrototype,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Default)]
pub enum Route {
    #[default]
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
    #[at("/auth")]
    Auth,
    #[at("/banners")]
    Banners,
    #[at("/forms")]
    Forms,
    #[at("/tables")]
    Tables,
    #[at("/dual-list")]
    DualList,
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
        Route::Auth => html! { <Auth /> },
        Route::Banners => html! { <Banners /> },
        Route::Forms => html! { <PForms /> },
        Route::Tables => html! { <TablesPrototype /> },
        Route::DualList => html! { <DualList /> },
    }
}
