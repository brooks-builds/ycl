use ::yew::prelude::*;
use stylist::{yew::styled_component, Style};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: String,
    pub alt: String,
    pub width: Option<u8>,
    pub height: Option<u8>,
}

#[styled_component(BBIcon)]
pub fn component(props: &Props) -> Html {
    let width = if let Some(width) = &props.width {
        Some(Style::new(css!(r#"width: ${w}px;"#, w = width)).unwrap())
    } else {
        None
    };

    let height = if let Some(height) = &props.height {
        Some(Style::new(css!(r#"height: ${h}px;"#, h = height)).unwrap())
    } else {
        None
    };

    html! {
        <img
            src={props.src.clone()}
            alt={props.alt.clone()}
            class={classes!("img-fluid", "img-thumbnail", width, height)} />
    }
}
