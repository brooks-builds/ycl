use std::fmt::Display;

use gloo::console::info;
use yew::prelude::*;

use super::icon::{BBIcon, BBIconSize, BBIconType};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| BBButtonStyle::Text)]
    pub button_style: BBButtonStyle,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub debug: bool,
    #[prop_or_default]
    pub debug_name: AttrValue,
    #[prop_or_default]
    pub action_icon: Option<BBIconType>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| BBButtonType::Button)]
    pub button_type: BBButtonType,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(BBButton)]
pub fn component(props: &Props) -> Html {
    let class = classes!(props.button_style.class(), props.classes.clone(),);
    let onclick = {
        let prop_onclick = props.onclick.clone();
        let debug = props.debug;
        let debug_name = props.debug_name.clone();

        Callback::from(move |_event: MouseEvent| {
            if debug {
                info!(format!("Button {debug_name} clicked"));
            }
            prop_onclick.emit(());
        })
    };

    html! {
        <button {class} {onclick} type={props.button_type.to_string()} disabled={props.disabled}>
            {
                props
                    .action_icon
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
pub enum BBButtonStyle {
    Text,
    PrimaryLight,
}

impl BBButtonStyle {
    pub fn class(&self) -> &'static str {
        match self {
            BBButtonStyle::Text => "btn",
            BBButtonStyle::PrimaryLight => "btn btn-primary light",
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBButtonType {
    Button,
    Submit,
    Reset,
}

impl Display for BBButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Button => "button",
                Self::Submit => "submit",
                Self::Reset => "reset",
            }
            .to_owned()
        )
    }
}
