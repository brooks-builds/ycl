use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::foundations::container::BBContainer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    #[prop_or_else(|| 3)]
    pub rows: u8,
    pub name: AttrValue,
    #[prop_or_default()]
    pub value: AttrValue,
    #[prop_or_default()]
    pub oninput: Callback<AttrValue>,
}

#[function_component(BBTextArea)]
pub fn component(props: &Props) -> Html {
    let oninput = {
        let props_oninput = props.oninput.clone();

        Callback::from(move |input_value: InputEvent| {
            let target = input_value
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>();
            props_oninput.emit(AttrValue::from(target.value()));
        })
    };

    html! {
        <BBContainer>
            <label for={props.id.clone()} class="form-label">{&props.label}</label>
            <textarea
                class="form-control"
                id={props.id.clone()}
                rows={props.rows.to_string()}
                name={props.name.clone()}
                value={props.value.clone()}
                {oninput}
            ></textarea>
        </BBContainer>
    }
}
