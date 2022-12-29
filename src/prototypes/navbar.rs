use super::app::wrap_in_accordian_item;
use crate::{
    elements::{icon::BBIcon, nav_link::BBNavLink},
    modules::{
        nav_links::{BBNavLinkPosition, BBNavLinks},
        navbar::BBNavbar,
    },
};
use yew::{html, virtual_dom::VNode};

pub fn add_navbar(accordian_items: &mut Vec<VNode>) {
    let brand = html! {
        <a href="#">
            <BBIcon
                src="/logo-bb-blue.svg"
                alt="Brooks Builds logo"
                width={150} />
        </a>
    };

    let center = html! {
        <BBNavLinks  position={BBNavLinkPosition::Center}>
            <BBNavLink
                href="#"
                current={true} >
                {"Home"}
            </BBNavLink>
            <BBNavLink
                href="#" >
                {"Another"}
            </BBNavLink>
            <BBNavLink
                href="#" >
                {"Link"}
            </BBNavLink>
        </BBNavLinks>
    };

    let right = html! {
        <BBNavLinks position={BBNavLinkPosition::Right}>
            <BBNavLink
                href="#"
                current={true} >
                {"Right Link"}
            </BBNavLink>
        </BBNavLinks>
    };

    let navbar = html! {
        <BBNavbar
            {brand}
            {center}
            {right} />
    };
    accordian_items.push(wrap_in_accordian_item(
        navbar,
        "navbar".to_owned(),
        "navbar".to_owned(),
    ));
}
