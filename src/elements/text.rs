use yew::prelude::*;

use crate::foundations::align_text::AlignText;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub align: AlignText,
}

#[function_component(BBText)]
pub fn component(props: &Props) -> Html {
    let classes = classes!(props.classes.clone(), props.align.class());

    html! {
        <p class={classes}>
            {props.children.clone()}
        </p>
    }
}
