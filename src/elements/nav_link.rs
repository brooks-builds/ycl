use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub current: Option<bool>,
    pub aria_current: Option<String>,
    pub href: String,
}

#[function_component(BBNavLink)]
pub fn component(props: &Props) -> Html {
    let active_class = props.current.map(|_| "active");

    html! {
        <li class="nav-item">
            <a
                class={classes!("nav-link", active_class)}
                aria-current={props.aria_current.clone()}
                href={props.href.clone()}>
                {props.children.clone()}
            </a>
        </li>
    }
}
