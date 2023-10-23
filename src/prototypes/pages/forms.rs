use crate::{
    elements::{
        button::{BBButton, BBButtonType},
        checkbox::BBCheckbox,
        form::BBForm,
        input::{BBInput, BBInputType},
        text_area::BBTextArea,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};
use std::ops::Deref;
use web_sys::FormData;
use yew::prelude::*;

#[function_component(PForms)]
pub fn component() -> Html {
    let email = use_state(String::new);
    let password = use_state(String::new);
    let checkbox = use_state(|| false);
    let checked_checkbox = use_state(|| false);
    let create_account_onsubmit = {
        let email_state = email.clone();
        let password_state = password.clone();
        let checkbox_state = checkbox.clone();
        let checked_checkbox_state = checked_checkbox.clone();

        Callback::from(move |form_data: FormData| {
            let email = form_data.get("email").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();
            let checkbox = form_data.get("checkbox").as_string();
            let checked_checkbox = form_data.get("checked_checkbox").as_string();

            email_state.set(email);
            password_state.set(password);

            if let Some(_value) = checkbox {
                checkbox_state.set(true);
            } else {
                checkbox_state.set(false);
            };
            if let Some(_value) = checked_checkbox {
                checked_checkbox_state.set(true);
            } else {
                checked_checkbox_state.set(false);
            }
        })
    };

    let textarea_state = use_state(|| AttrValue::from(""));
    let textarea_oninput = {
        let state = textarea_state.clone();

        Callback::from(move |value: AttrValue| {
            state.set(value);
        })
    };
    let textarea_onsubmit = {
        let textarea_value = textarea_state.clone();
        Callback::from(move |formdata| {
            gloo::console::log!(formdata);
            textarea_value.set(AttrValue::from(""));
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
                        name="email"
                        message="what email address do you want to use?"
                    />
                    <BBInput
                        label="Password"
                        id="password"
                        input_type={BBInputType::Password}
                        name="password"
                        message="Password must have 8 characters, at least 2 special characters, and 1 number"
                    />
                    <BBCheckbox
                        id="checkbox"
                        label="checkbox"
                        name="checkbox"
                        value="checkbox checked"
                    />
                    <BBCheckbox
                        checked={true}
                        id="checked_checkbox"
                        label="checked_checkbox"
                        name="checked_checkbox"
                        value="checked_checkbox checked"
                    />
                    <BBContainer>
                        <BBButton button_type={BBButtonType::Submit}>{"Create Account"}</BBButton>
                    </BBContainer>
                </BBForm>
                <BBContainer>
                    <p>{format!("email: {}", &*email)}</p>
                    <p>{format!("password: {}", &*password)}</p>
                    <p>{format!("checkbox: {}", &*checkbox)}</p>
                    <p>{format!("checked_checkbox: {}", &*checked_checkbox)}</p>
                </BBContainer>
            </BBContainer>

            <BBContainer>
                <BBForm onsubmit={textarea_onsubmit}>
                    <BBTitle level={BBTitleLevel::Two}>{"Text Area"}</BBTitle>
                    <BBTextArea
                        id="text-area"
                        label="Should empty on submit"
                        name="textarea"
                        value={textarea_state.deref().clone()}
                        oninput={textarea_oninput}
                    />

                    <BBButton button_type={BBButtonType::Submit}>{"Submit"}</BBButton>
                </BBForm>
            </BBContainer>
        </BBContainer>
    }
}
