use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use crate::foundations::align_text::AlignText;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: AttrValue,
    pub title: AttrValue,
    #[prop_or_default]
    pub align: AlignText,
    #[prop_or_else(|| 560)]
    pub width: u32,
    #[prop_or_else(|| 315)]
    pub height: u32,
}

#[styled_component(BBYouTubeVideo)]
pub fn component(props: &Props) -> Html {
    let style = Style::new(css!(
        r#"
        position: relative;
        padding-bottom: 56.25%; /* 16:9 */
        height: 0;

        iframe {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }
    "#
    ))
    .unwrap();
    html! {
        <div class={classes!(props.align.class(), style)}>
            <iframe
                width={format!("{}", props.width)}
                height={format!("{}", props.height)}
                src={props.src.clone()}
                title={props.title.clone()}
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen={true}></iframe>
        </div>
    }
}
