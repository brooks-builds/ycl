use crate::{
    elements::{
        accordian_item::BBAccordianItem,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::align_text::AlignText,
    modules::accordian::BBAccordian,
};
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn component() -> Html {
    let children = vec![html! {
        <BBAccordianItem title="Title Level 1" title_level={BBTitleLevel::Two}>
            <BBTitle level={BBTitleLevel::One}>{"Title Level 1"}</BBTitle>
        </BBAccordianItem>
    }];
    html! {
        <>
            <BBTitle
            level={BBTitleLevel::One}
            align={AlignText::Center} >
                {"Brooks Builds Yew Component Library"}
            </BBTitle>
            <BBAccordian>
                {children}
            </BBAccordian>
        </>
    }
}
