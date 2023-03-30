use yew::prelude::*;

use crate::foundations::container::BBContainer;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    #[prop_or_else(|| 3)]
    pub rows: u8,
}

#[function_component(BBTextArea)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer>
            <label for={props.id.clone()} class="form-label">{&props.label}</label>
            <textarea class="form-control" id={props.id.clone()} rows={props.rows.to_string()}></textarea>
        </BBContainer>
    }
}
