use super::navbar_link::BBNavbarLink;
use crate::elements::{
    icon::{BBIcon, BBIconType},
    text::BBText,
};
use ::yew::prelude::*;
use yew_router::{prelude::Link, Routable};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub links: Vec<BBNavbarLink<T>>,
    pub login_route: T,
    pub create_account_route: T,
    pub show_brand: Option<bool>,
    #[prop_or_default]
    pub username: AttrValue,
    #[prop_or_default]
    pub is_authenticated: bool,
}

#[function_component(BBNavbar)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary" role="navigation">
            <div class="container-fluid">
                {
                    props.show_brand.clone().map(|_brand| {
                        html! {
                            <BBIcon icon_type={BBIconType::Brand} />
                        }
                    })
                }
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <div class="mx-auto"></div>
                    <ul class="navbar-nav auto mb-2 mb-lg-0">
                        {
                            props.links.clone().into_iter().map(|link| {
                                html! {
                                    <li class="nav-item">
                                        <Link<T> to={link.clone().to} classes={link.clone().classes()}>{link.label}</Link<T>>
                                    </li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                    <div class="mx-auto"></div>
                    {
                        if props.is_authenticated {
                            html! {
                                <ul class="navbar-nav ml-0 mb-2 mb-lg-0">
                                    <li class="nav-item">
                                        <BBText>{format!("Welcome {}", &props.username)}</BBText>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {
                                <ul class="navbar-nav ml-0 mb-2 mb-lg-0">
                                    <li class="nav-item">
                                        <Link<T> to={props.login_route.clone()} classes="nav-link">{"Login"}</Link<T>>
                                    </li>
                                    <li class="nav-item">
                                        <Link<T> to={props.create_account_route.clone()} classes="nav-link btn btn-primary">{"Get Started"}</Link<T>>
                                    </li>
                                </ul>
                            }
                        }
                    }
                </div>
            </div>
        </nav>
    }
}
