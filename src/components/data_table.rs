#![allow(non_camel_case_types)]
use yew::prelude::*;

use crate::{
    components::data_table_title::BBDataTableTitle, elements::title::BBTitleLevel,
    foundations::container::BBContainer,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_else(|| BBTitleLevel::Two)]
    pub title_level: BBTitleLevel,
    pub id: AttrValue,
}

#[function_component(BBDataTable)]
pub fn component(props: &Props) -> Html {
    let onsearch = Callback::from(|search_text: AttrValue| {
        gloo::console::log!("searching for:", search_text.as_str());
    });

    html! {
        <BBContainer>
            <BBContainer>
                <BBDataTableTitle
                    title={props.title.clone()}
                    title_level={props.title_level}
                    id={props.id.clone()}
                    {onsearch}
                />
            </BBContainer>
        </BBContainer>
    }
}
