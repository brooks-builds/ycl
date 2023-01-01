use ::yew::prelude::*;
use stylist::{yew::styled_component, Style};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon_type: BBIconType,
}

#[styled_component(BBIcon)]
pub fn component(props: &Props) -> Html {
    let style = Style::new(css!(
        r#"
        width: 150px;
    "#
    ))
    .unwrap();

    html! {
        <img
            src={props.icon_type.src()}
            alt={props.icon_type.alt()}
            class={classes!("img-fluid", "img-thumbnail", style)} />
    }
}

#[derive(PartialEq)]
pub enum BBIconType {
    Brand,
}

impl BBIconType {
    pub fn src(&self) -> &'static str {
        match self {
            BBIconType::Brand => "/logo-bb-blue.svg",
        }
    }

    pub fn alt(&self) -> &'static str {
        match self {
            BBIconType::Brand => "Brooks Builds Brand icon",
        }
    }
}
