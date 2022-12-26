use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn component() -> Html {
    html! {
        <>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Brooks Builds Yew Component Library"}</BBTitle>
        </>
    }
}
