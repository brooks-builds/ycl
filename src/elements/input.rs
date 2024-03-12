use std::fmt::Display;

use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    components::button_icon::BBButtonIcon, elements::icon::BBIconType,
    foundations::states::BBValidationState,
};

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
    pub pattern: Option<AttrValue>,
    #[prop_or_default]
    pub oninput: Callback<AttrValue>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub show_clear: bool,
    #[prop_or_default]
    pub onclear: Callback<()>,
    #[prop_or_default]
    pub classes: Classes,
}

#[styled_component(BBInput)]
pub fn component(props: &Props) -> Html {
    let debounce = use_state(|| None);

    let props_onisvalid = props.onisvalid.clone();
    let debounce_time = props.validation_debounce;
    let props_oninput = props.oninput.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        let input_value = AttrValue::from(input_element.value());
        let props_onisvalid = props_onisvalid.clone();
        let timer = gloo::timers::callback::Timeout::new(debounce_time, move || {
            let is_valid = input_element.check_validity();
            props_onisvalid.emit(is_valid.into());
        });
        debounce.set(Some(timer));
        props_oninput.emit(input_value);
    });
    let label_classes = classes!(
        "form-label",
        if props.inline {
            Some("visually-hidden")
        } else {
            None
        },
    );
    let input_group_class = classes!(if props.inline {
        Some("input-group")
    } else {
        None
    },);

    let onclear = {
        let props_onclear = props.onclear.clone();

        Callback::from(move |_event| props_onclear.emit(()))
    };

    let input_classes = classes!(props.classes.clone(), "form-control");

    html! {
        <div class={input_group_class}>
            <label for={props.id.clone()} class={label_classes}>{create_label(props.label.clone(), props.required)}</label>
            if props.inline {
                <span class="input-group-text">{props.label.clone()}</span>
            }
            <input
                type={props.input_type.to_string()}
                class={input_classes}
                id={props.id.clone()}
                required={props.required}
                value={props.value.clone()}
                name={props.name.clone()}
                {oninput}
                pattern={props.pattern.clone()}
                minlength={props.min.map(|min| min.to_string())}
                maxlength={props.max.map(|max| max.to_string())}
            />
            if props.show_clear {
                <BBButtonIcon icon_type={BBIconType::Clear} shift_left={true} onclick={onclear} label="Clear search" />
        }
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
    let required_text = if required { "*" } else { "" };

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
