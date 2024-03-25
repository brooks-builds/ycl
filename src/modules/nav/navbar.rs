#![allow(non_camel_case_types)]

use super::navbar_link::BBNavbarLink;
use crate::{
    elements::{
        external_link::BBLink,
        icon::{BBIcon, BBIconType},
        pill::BBPill,
    },
    foundations::{color::BBColor, roles::BBRole},
};
use ::yew::prelude::*;
use yew_router::{prelude::Link, Routable};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub links: Vec<BBNavbarLink<T>>,
    pub show_brand: Option<bool>,
    pub username: Option<AttrValue>,
    #[prop_or_default]
    pub is_authenticated: bool,
    #[prop_or_default]
    pub logout_url: AttrValue,
    #[prop_or_default]
    pub logout_onclick: Callback<()>,
    pub role: Option<BBRole>,
    #[prop_or_default]
    pub onlogin_click: Callback<()>,
}

#[function_component(BBNavbar)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let login_onclick = {
        let onlogin_click = props.onlogin_click.clone();

        Callback::from(move |_event| {
            onlogin_click.emit(());
        })
    };

    html! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary" role="navigation">
            <div class="container-fluid">
                {
                    props.show_brand.map(|_brand| {
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
                                    <li class="nav-item navbar-text">
                                        <span>{format!("Welcome {}", props.username.clone().unwrap_or_else(|| AttrValue::from("Learner")))}</span>
                                    </li>
                                    {
                                        if let Some(role) = props.role {
                                            html! {
                                                <li class="nav-item navbar-text mx-1">
                                                    <BBPill color={BBColor::Success}>{role.to_string()}</BBPill>
                                                </li>
                                            }
                                        } else { html! {}}
                                    }
                                    <li class="nav-item mx-1">
                                        <BBLink href={props.logout_url.clone()} button={true} classes="nav-link" onclick={props.logout_onclick.clone()} prevent_default={true}>{"logout"}</BBLink>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {
                                <ul class="navbar-nav ml-0 mb-2 mb-lg-0">
                                    <BBLink href={"/"} onclick={login_onclick} prevent_default={true}>{"Login or Signup"}</BBLink>
                                    // <li class="nav-item">
                                    //     <a href={props.login_uri.clone()} classes="nav-link" target="_self">{"Login or Create Account"}</a>
                                    // </li>
                                </ul>
                            }
                        }
                    }
                </div>
            </div>
        </nav>
    }
}
