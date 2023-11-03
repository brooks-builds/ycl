use yew::prelude::*;

use crate::{
    components::dual_list::BBDualList,
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{align_text::AlignText, container::BBContainer},
};

#[function_component]
pub fn DualListPrototype() -> Html {
    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Dual Lists"}</BBTitle>
            <BBDualList
                available_title="Available Courses"
                selected_title="SQLx Rust Articles"
                title_level={BBTitleLevel::Two}
                id="course-articles"
            />
        </BBContainer>
    }
}
