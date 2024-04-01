use super::navbar_link::BBNavbarLink;
use crate::{
    elements::nav::BBNav,
    foundations::container::{BBContainer, BBContainerMargin},
};
use yew::prelude::*;
use yew_router::{prelude::Link, Routable};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub left_links: Vec<BBNavbarLink<T>>,
    pub right_links: Vec<BBNavbarLink<T>>,
}

#[function_component(BBNavFooter)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    html! {
        <BBNav aria_label="footer nav">
            <BBContainer margin={BBContainerMargin::Fluid}>
                <div>
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        {
                            props
                                .left_links
                                .clone()
                                .into_iter()
                                .map(|link| {
                                    let active = if link.active {
                                        Some("active")
                                    } else {
                                        None
                                    };

                                    html! {
                                        <li class="nav-item">
                                            <Link<T> to={link.to} classes={classes!("nav-link", active)}>{link.label}</Link<T>>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </ul>
                    <div class="mx-auto"></div>
                    <ul class="navbar-nav ml-0 mb-2 mb-lg-0">
                        {
                            props
                                .right_links
                                .clone()
                                .into_iter()
                                .map(|link| {
                                    html! {
                                        <li class="nav-item">
                                            <Link<T> to={link.to} classes={classes!("nav-link")}>{link.label}</Link<T>>
                                        </li>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </ul>
                </div>
            </BBContainer>
        </BBNav>
    }
}
