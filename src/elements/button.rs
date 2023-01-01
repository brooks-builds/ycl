use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| BBButtonType::Text)]
    pub button_type: BBButtonType,
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component(BBButton)]
pub fn component(props: &Props) -> Html {
    let class = props.button_type.class();
    let onclick = {
        let prop_onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            prop_onclick.emit(());
        })
    };

    html! {
        <button {class} {onclick}>{props.children.clone()}</button>
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBButtonType {
    Text,
}

impl BBButtonType {
    pub fn class(&self) -> &'static str {
        match self {
            BBButtonType::Text => "btn",
        }
    }
}
