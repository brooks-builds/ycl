use stylist::yew::styled_component;
use yew::prelude::*;

use crate::{
    elements::{
        input::BBInput,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{align_text::AlignText, container::BBContainer},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub title_level: BBTitleLevel,
    #[prop_or_default]
    pub border: bool,
    pub id: AttrValue,
}

#[styled_component]
pub fn BBList(props: &Props) -> Html {
    let border_css = if props.border {
        Some(css!("border: 1px solid black;"))
    } else {
        None
    };
    let class = classes!(border_css);
    let search_id = format!("{}-search", props.id.as_str());

    html! {
        <BBContainer classes={class}>
            <BBTitle level={props.title_level.clone()} align={AlignText::Center}>
                {props.title.clone()}
            </BBTitle>
            <BBInput
                id={search_id}
                label="Search"
                name="search"
            />
        </BBContainer>
    }
}
