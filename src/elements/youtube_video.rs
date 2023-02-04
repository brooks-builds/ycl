use stylist::yew::styled_component;
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
    html! {
        <div class={classes!(props.align.class())}>
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
