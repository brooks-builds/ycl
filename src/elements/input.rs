use std::{fmt::Display, ops::Deref};

use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::foundations::states::BBValidationState;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    #[prop_or_else(|| BBInputType::Text)]
    pub input_type: BBInputType,
    #[prop_or_default]
    pub required: bool,
    pub name: AttrValue,
    #[prop_or_default]
    pub message: AttrValue,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub onisvalid: Callback<BBValidationState>,
    #[prop_or_else(|| 750)]
    pub validation_debounce: u32,
    #[prop_or_default]
    pub is_valid: BBValidationState,
}

#[styled_component(BBInput)]
pub fn component(props: &Props) -> Html {
    let prop_value = props.value.clone();
    let value = use_state(move || AttrValue::from(""));
    let debounce = use_state(|| None);

    let use_effect_value = value.clone();
    use_effect_once(move || {
        use_effect_value.set(prop_value);

        || {}
    });

    let oninput_value = value.clone();
    let props_onisvalid = props.onisvalid.clone();
    let debounce_time = props.validation_debounce;
    let oninput = Callback::from(move |event: InputEvent| {
        let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        let input_value = input_element.value();
        oninput_value.set(input_value.into());
        let props_onisvalid = props_onisvalid.clone();
        let timer = gloo::timers::callback::Timeout::new(debounce_time, move || {
            let is_valid = input_element.check_validity();
            props_onisvalid.emit(is_valid.into());
        });
        debounce.set(Some(timer));
    });

    html! {
        <div>
            <label for={props.id.clone()} class="form-label">{create_label(props.label.clone(), props.required)}</label>
            <input
                type={props.input_type.to_string()}
                class="form-control"
                id={props.id.clone()}
                required={props.required}
                value={value.deref().clone()}
                name={props.name.clone()}
                pattern="abc"
                {oninput}
            />
            <div id={format!("{}-message", &props.id)} class="form-text">{props.message.clone()}</div>
            {
                create_error_message(props.input_type, props.required, &props.is_valid)
                    .map(|error_message| {
                        let class = Style::new(css!("color: red;")).unwrap();

                        html! {
                            <div id={format!("{}-help", &props.id)} class={classes!("form-text", class)}>{error_message}</div>
                        }
                    })
            }
        </div>
    }
}

fn create_label(label: AttrValue, required: bool) -> String {
    let required_text = if required { "(required)" } else { "" };

    format!("{label} {required_text}")
}

fn create_error_message(
    input_type: BBInputType,
    required: bool,
    validation_state: &BBValidationState,
) -> Option<String> {
    let mut error_messages = vec![];

    match validation_state {
        BBValidationState::Initialized => None,
        BBValidationState::Valid => None,
        BBValidationState::NotValid => {
            let type_error_message = match input_type {
                BBInputType::Email => "must be an email",
                _ => "",
            };

            if !type_error_message.is_empty() {
                error_messages.push(type_error_message);
            }

            if required {
                error_messages.push("required");
            }

            let error_message = error_messages.join(" and ");
            Some(error_message)
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBInputType {
    Email,
    Password,
    Text,
    Number,
    Checkbox,
}

impl Display for BBInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Email => "email",
            Self::Password => "password",
            Self::Text => "text",
            Self::Number => "number",
            BBInputType::Checkbox => "checkbox",
        })
    }
}

pub struct BBInputValue {
    pub value: AttrValue,
    pub is_valid: bool,
}
