use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel, _Props::level},
    foundations::{align_text::AlignText, column::BBCol, container::BBContainer, row::BBRow},
};

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub title: AttrValue,
}

#[function_component()]
pub fn BBDualList(props: &Props) -> Html {
    html! {
        <BBContainer>
            <BBRow>
                <BBCol>
                    <BBTitle
                        level={BBTitleLevel::One}
                        align={AlignText::Center}
                    >
                        {props.title.clone()}
                    </BBTitle>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}
