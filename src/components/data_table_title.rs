#![allow(non_camel_case_types)]
use std::ops::Deref;

use yew::prelude::*;

use crate::{
    elements::{
        input::BBInput,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{column::BBCol, container::BBContainer, row::BBRow},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub title_level: BBTitleLevel,
    pub id: AttrValue,
    pub onsearch: Callback<AttrValue>,
}

#[function_component(BBDataTableTitle)]
pub fn component(props: &Props) -> Html {
    let search_id = AttrValue::from(format!("{}-search", props.id.as_str()));
    let search_text = use_state(|| AttrValue::from(""));

    let search_oninput = {
        let search_text = search_text.clone();
        let onsearch = props.onsearch.clone();

        Callback::from(move |input_search_text: AttrValue| {
            search_text.set(input_search_text.clone());
            let lowercase_search_text = AttrValue::from(input_search_text.as_str().to_lowercase());
            onsearch.emit(lowercase_search_text);
        })
    };

    let onclear = {
        let search_text = search_text.clone();
        let onsearch = props.onsearch.clone();

        Callback::from(move |_event| {
            search_text.set(AttrValue::from(""));
            onsearch.emit(AttrValue::from(""));
        })
    };

    html! {
        <BBContainer>
            <BBRow>
                <BBCol>
                    <BBTitle level={props.title_level}>{props.title.clone()}</BBTitle>
                </BBCol>
                <BBCol>
                    <BBInput
                        id={search_id}
                        label="search"
                        name="search"
                        inline={true}
                        oninput={search_oninput}
                        value={search_text.deref().clone()}
                        show_clear={true}
                        {onclear}
                    />
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}
