use web_sys::FormData;
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
    let email = use_state(String::new);
    let password = use_state(String::new);
    let create_account_onsubmit = {
        let email_state = email.clone();
        let password_state = password.clone();

        Callback::from(move |form_data: FormData| {
            let email = form_data.get("email").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();

            email_state.set(email);
            password_state.set(password);
        })
    };

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
                        name="email"
                        message="what email address do you want to use?"
                    />
                    <BBInput
                        label="Password"
                        id="password"
                        input_type={BBInputType::Password}
                        required={true}
                        name="password"
                        message="Password must have 8 characters, at least 2 special characters, and 1 number"
                    />
                    <BBButton button_type={BBButtonType::Submit}>{"Create Account"}</BBButton>
                </BBForm>
                <BBContainer>
                    <p>{format!("email: {}", &*email)}</p>
                    <p>{format!("password: {}", &*password)}</p>
                </BBContainer>
            </BBContainer>
        </BBContainer>
    }
}
