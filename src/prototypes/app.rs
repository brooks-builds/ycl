use crate::{
    elements::{
        accordian_item::BBAccordianItem,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::align_text::AlignText,
    modules::accordian::BBAccordian,
    prototypes::{navbar::add_navbar, titles::add_titles},
};
use yew::{function_component, html, virtual_dom::VNode, Html};

#[function_component(App)]
pub fn component() -> Html {
    let mut children = vec![];

    add_titles(&mut children, "yew-components".to_owned());
    add_navbar(&mut children);

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

pub fn wrap_in_accordian_item(child: VNode, id: String, title: String) -> Html {
    html! {
        <BBAccordianItem
            {id}
            parent_id="yew-components"
            {title}
            title_level={BBTitleLevel::Two} >
            {child}
        </BBAccordianItem>
    }
}
