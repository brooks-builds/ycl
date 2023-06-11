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
    #[prop_or_default]
    pub target: BBLinkTarget,
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
    let target = props.target.to_string();

    html! {
        <a
            href={props.href.clone()}
            class={classes!(props.classes.clone(), button_classes)}
            {onclick}
            {target}
        >
            {props.children.clone()}
        </a>
    }
}

#[derive(PartialEq, Clone, Default)]
pub enum BBLinkTarget {
    #[default]
    Own,
    Blank,
}

impl ToString for BBLinkTarget {
    fn to_string(&self) -> String {
        match self {
            BBLinkTarget::Own => "_self",
            BBLinkTarget::Blank => "_blank",
        }
        .into()
    }
}
