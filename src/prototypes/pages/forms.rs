use yew::prelude::*;

use crate::{
    elements::{
        button::{BBButton, BBButtonType},
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
    let create_account_onsubmit = Callback::from(|_| {
        gloo::console::log!("form submitted");
    });

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
                <BBForm onsubmit={create_account_onsubmit}>
                    <BBInput
                        label="Email"
                        id="email"
                        input_type={BBInputType::Email}
                        required={true}
                    />
                    <BBInput
                        label="Password"
                        id="password"
                        input_type={BBInputType::Password}
                        required={true}
                    />
                    <button type="submit" class="btn">{"create account"}</button>
                    <BBButton button_type={BBButtonType::Submit}>{"other create account button"}</BBButton>
                </BBForm>
            </BBContainer>
        </BBContainer>
    }
}
