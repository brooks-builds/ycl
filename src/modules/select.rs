use std::cmp::Ordering;

use crate::foundations::container::BBContainer;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    pub options: Vec<BBOption>,
    pub name: AttrValue,
    #[prop_or_else(|| "----------".into())]
    pub select_name: AttrValue,
}

#[function_component(BBSelect)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer>
            <label for={props.id.clone()} class="form-label">{&props.label}</label>
            <select class="form-select" id={props.id.clone()} name={props.name.clone()} data-testid="select">
                <option value="">{props.select_name.clone()}</option>
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

impl PartialOrd<Self> for BBOption {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.value.as_str().partial_cmp(self.value.as_str())
    }
}
