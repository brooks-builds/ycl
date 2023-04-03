use crate::foundations::container::BBContainer;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    pub options: Vec<BBOption>,
    pub name: AttrValue,
}

#[function_component(BBSelect)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer>
            <label for={props.id.clone()} class="form-label">{&props.label}</label>
            <select class="form-select" id={props.id.clone()} name={props.name.clone()} data-testid="select">
                <option value="">{"Choose Tag"}</option>
                {
                 props.options.iter().map(|option| html!{ <option value={option.value.clone()}>{option.label.clone()}</option>}).collect::<Html>()
                }
            </select>
        </BBContainer>
    }
}

#[derive(PartialEq, Clone)]
pub struct BBOption {
    pub value: AttrValue,
    pub label: AttrValue,
}
