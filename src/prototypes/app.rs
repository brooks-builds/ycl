use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
    modules::accordian::BBAccordian,
    prototypes::titles::add_titles,
};
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn component() -> Html {
    let mut children = vec![];

    add_titles(&mut children, "yew-components".to_owned());

    html! {
        <>
            <BBTitle
            level={BBTitleLevel::One}
            align={AlignText::Center} >
                {"Brooks Builds Yew Component Library"}
            </BBTitle>
            <BBAccordian id="yew-components">
                {children}
            </BBAccordian>
        </>
    }
}
