use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub href: AttrValue,
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub button: bool,
}

#[styled_component(BBLink)]
pub fn component(props: &Props) -> Html {
    let button_classes = if props.button {
        Some(classes!("btn", "btn-primary"))
    } else {
        None
    };

    html! {
        <a
            href={props.href.clone()}
            class={classes!(props.classes.clone(), button_classes)}
        >
            {props.children.clone()}
        </a>
    }
}
