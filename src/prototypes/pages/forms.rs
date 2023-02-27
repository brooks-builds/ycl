use yew::prelude::*;

use crate::{
    elements::{
        button::BBButton,
        form::BBForm,
        input::{BBInput, BBInputType},
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};

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
                <BBForm>
                    <BBInput
                        label="Email"
                        id="email"
                        input_type={BBInputType::Email}
                        required={true}
                    />
                    <BBButton>{"Create Account"}</BBButton>
                </BBForm>
            </BBContainer>
        </BBContainer>
    }
}
