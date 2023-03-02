use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default()]
    pub children: Children,
    #[prop_or_default]
    pub onsubmit: Callback<FormData>,
}

#[function_component(BBForm)]
pub fn component(props: &Props) -> Html {
    let onsubmit = {
        let prop_onsubmit = props.onsubmit.clone();

        Callback::from(move |event: SubmitEvent| {
            let form = event.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&form).unwrap();

            event.prevent_default();
            prop_onsubmit.emit(form_data);
        })
    };

    html! {
        <form onsubmit={onsubmit} method="GET" action="/">{props.children.clone()}</form>
    }
}
