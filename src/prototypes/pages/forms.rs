use yew::prelude::*;

use crate::{foundations::{container::{BBContainer, BBContainerMargin}, align_text::AlignText}, elements::{title::{BBTitle, BBTitleLevel}, form::BBForm}};

#[function_component(PForms)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BBTitle 
                level={BBTitleLevel::One}
                align={AlignText::Center}
            >
                {"Forms"}
            </BBTitle>
            <BBContainer margin={BBContainerMargin::Normal}>
                <BBTitle level={BBTitleLevel::Two}>{"Create Account"}</BBTitle>
                <BBForm></BBForm>
            </BBContainer>
        </BBContainer>
    }
}
