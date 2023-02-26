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
    #[prop_or_default]
    pub prevent_default: bool,
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[styled_component(BBLink)]
pub fn component(props: &Props) -> Html {
    let button_classes = if props.button {
        Some(classes!("btn", "btn-primary"))
    } else {
        None
    };

    let prevent_default = props.prevent_default;
    let props_onclick = props.onclick.clone();
    let onclick = Callback::from(move |event: MouseEvent| {
        if prevent_default {
            event.prevent_default();
        }

        props_onclick.emit(());
    });

    html! {
        <a
            href={props.href.clone()}
            class={classes!(props.classes.clone(), button_classes)}
            {onclick}
        >
            {props.children.clone()}
        </a>
    }
}
