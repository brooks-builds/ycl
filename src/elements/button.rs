use gloo::console::log;
use yew::prelude::*;

use super::icon::{BBIconType, BBIcon, BBIconSize};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| BBButtonType::Text)]
    pub button_type: BBButtonType,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub debug: bool,
    #[prop_or_default]
    pub debug_name: AttrValue,
    pub action_icon: Option<BBIconType>
}

#[function_component(BBButton)]
pub fn component(props: &Props) -> Html {
    let class = props.button_type.class();
    let onclick = {
        let prop_onclick = props.onclick.clone();
        let debug = props.debug;
        let debug_name = props.debug_name.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if debug {
                log!(format!("Button {debug_name} clicked"));
            }
            prop_onclick.emit(());
        })
    };

    html! {
        <button {class} {onclick}>
            {
                props
                    .action_icon
                    .clone()
                    .map(|icon_type| {
                        html! {
                            <BBIcon 
                                {icon_type}
                                size={BBIconSize::Smaller} 
                                classes="me-2" />
                        }
                    })
            }
            {props.children.clone()}
        </button>
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBButtonType {
    Text,
    PrimaryLight,
}

impl BBButtonType {
    pub fn class(&self) -> &'static str {
        match self {
            BBButtonType::Text => "btn",
            BBButtonType::PrimaryLight => "btn btn-primary light",
        }
    }
}
