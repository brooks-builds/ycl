use yew::prelude::*;
use yew_router::{prelude::Link, Routable};

use super::navbar_link::BBNavbarLink;

#[derive(Properties, PartialEq)]
pub struct Props<T: Routable + 'static> {
    pub links: Vec<BBNavbarLink<T>>,
}

#[function_component(BBAdminNav)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    html! {
        <ul class="nav">
          {
            props.links.iter().map(|link| {
              html! {
                <li class="nav-item">
                  <Link<T> classes="nav-link" to={link.to.clone()}>{link.label}</Link<T>>
                </li>
              }
            }).collect::<Vec<Html>>()
          }
        </ul>
    }
}
