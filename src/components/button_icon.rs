#![allow(non_camel_case_types)]
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::elements::icon::{BBIcon, BBIconSize, BBIconType};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon_type: BBIconType,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub shift_left: bool,
    pub label: AttrValue,
}

#[styled_component(BBButtonIcon)]
pub fn component(props: &Props) -> Html {
    let shift_left_style = if props.shift_left {
        Style::new(css!(
            "margin-left: -40px !important; z-index: 100 !important;"
        ))
        .ok()
    } else {
        None
    };
    let class = classes!("btn", "bg-transparent", shift_left_style);
    let onclick = {
        let props_onclick = props.onclick.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            props_onclick.emit(());
        })
    };

    html! {
        <button
            type="button"
            {class}
            {onclick}
            aria-label={props.label.clone()}
        >
            <BBIcon
                icon_type={props.icon_type}
                size={BBIconSize::Tiny}
                outline={false}
            />
        </button>
    }
}
