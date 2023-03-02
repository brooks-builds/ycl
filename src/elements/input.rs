use std::{fmt::Display, ops::Deref};

use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    #[prop_or_else(|| BBInputType::Text)]
    pub input_type: BBInputType,
    #[prop_or_default]
    pub required: bool,
    pub name: AttrValue,
}

#[styled_component(BBInput)]
pub fn component(props: &Props) -> Html {
    let is_valid = use_state(|| true);
    let value = use_state(|| String::new());

    let onchange = {
        let is_valid = is_valid.clone();
        let value = value.clone();

        Callback::from(move |event: Event| {
            let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            is_valid.set(input_element.check_validity());
            value.set(input_element.value());
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
            />
            {
                create_error_message(props.input_type, props.required, *is_valid)
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

fn create_error_message(input_type: BBInputType, required: bool, is_valid: bool) -> Option<String> {
    let mut error_messages = vec![];

    if !is_valid {
        let type_error_message = match input_type {
            BBInputType::Email => "must be an email",
            _ => "",
        };

        error_messages.push(type_error_message);

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
}

impl Display for BBInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Email => "email",
            Self::Password => "password",
            Self::Text => "text",
        })
    }
}
