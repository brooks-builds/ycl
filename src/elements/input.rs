use std::{fmt::Display, ops::Deref};

use stylist::yew::styled_component;
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
}

#[styled_component(BBInput)]
pub fn component(props: &Props) -> Html {
    todo!("set error message in dom when invalid");
    let value = use_state(|| String::new());
    let onchange = {
        let value = value.clone();
        Callback::from(move |event: Event| {
            let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let is_valid = input_element.check_validity();
            gloo::console::log!("is valid", is_valid);
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
                {onchange}
            />
            {
                error_message(&*value, props.required)
                    .map(|error_message| {
                        html! {
                            <div id={format!("{}-help", &props.id)} class="form-text">{error_message}</div>
                        }
                    })
            }
        </div>
    }
}

#[derive(PartialEq)]
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
