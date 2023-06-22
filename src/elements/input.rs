use std::{fmt::Display, ops::Deref};

use gloo::timers::callback::Timeout;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_effect_update, use_renders_count};

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
    pub onchange: Callback<BBInputValue>,
    #[prop_or_default]
    pub oninput: Callback<BBInputValue>,
}

#[styled_component(BBInput)]
pub fn component(props: &Props) -> Html {
    let is_valid = use_state(|| false);
    let prop_value = props.value.clone();
    let value = use_state(move || prop_value);
    let validate = use_state(|| false);
    let debounce = use_state(|| None);

    let onchange = {
        let is_valid = is_valid.clone();
        let value = value.clone();
        let props_onchange = props.onchange.clone();

        Callback::from(move |event: Event| {
            let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let valid = input_element.check_validity();
            is_valid.set(valid);
            let input_value: AttrValue = input_element.value().into();
            value.set(input_value.clone());
            props_onchange.emit(BBInputValue {
                value: input_value,
                is_valid: valid,
            });
        })
    };

    let oninput = {
        let prop_oninput = props.oninput.clone();
        let value = value.clone();
        let debounce = debounce.clone();
        let is_valid = is_valid.clone();

        // TODO: maybe implement debouncing here for invalid check. Think about when and where that check should happen
        Callback::from(move |event: InputEvent| {
            let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            value.set(input_element.value().into());

            let timer = Timeout::new(1000, move || {
                let valid = input_element.check_validity();
                prop_oninput.clone().emit(BBInputValue {
                    value: input_element.clone().value().into(),
                    is_valid: valid.clone(),
                });
            });
        })
    };

    {
        let prop_value = props.value.clone();
        let value = value.clone();

        use_effect_update(move || {
            let state_value = value.deref().clone();
            if state_value != prop_value {
                value.set(prop_value);
            }

            || {}
        });
    }

    let oninvalid = {
        let validate = validate.clone();

        Callback::from(move |_event: Event| {
            validate.set(true);
        })
    };

    html! {

        <div>
            <label for={props.id.clone()} class="form-label">{props.label.clone()}</label>
            <input
                type={props.input_type.to_string()}
                class="form-control"
                id={props.id.clone()}
                required={props.required}
                value={value.deref().clone()}
                name={props.name.clone()}
                {onchange}
                {oninvalid}
            />
            <div id={format!("{}-message", &props.id)} class="form-text">{props.message.clone()}</div>
            {
                create_error_message(props.input_type, props.required, *is_valid, *validate)
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

fn create_error_message(
    input_type: BBInputType,
    required: bool,
    is_valid: bool,
    validate: bool,
) -> Option<String> {
    if !validate {
        return None;
    }

    let mut error_messages = vec![];

    if !is_valid {
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
    } else {
        None
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
