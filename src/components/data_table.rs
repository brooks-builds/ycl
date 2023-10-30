#![allow(non_camel_case_types)]
use std::{collections::HashMap, ops::Deref};

use crate::{
    components::data_table_title::BBDataTableTitle,
    elements::{table::BBTable, title::BBTitleLevel},
    foundations::container::BBContainer,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_else(|| BBTitleLevel::Two)]
    pub title_level: BBTitleLevel,
    pub id: AttrValue,
    pub data: BBDataTableData,
}

#[function_component(BBDataTable)]
pub fn component(props: &Props) -> Html {
    let searched_values_state = {
        let props_data_values = props.data.values.clone();
        use_state(move || props_data_values)
    };

    let onsearch = {
        let searched_values_state = searched_values_state.clone();
        let props_values = props.data.values.clone();

        Callback::from(move |search_text: AttrValue| {
            let searched_values = props_values
                .clone()
                .into_iter()
                .filter(|prop_row| {
                    for value in prop_row.values() {
                        if value.as_str().to_lowercase().contains(search_text.as_str()) {
                            return true;
                        }
                    }

                    return false;
                })
                .collect::<BBDataTableValues>();

            searched_values_state.set(searched_values);
        })
    };

    html! {
        <BBContainer>
            <BBContainer>
                <BBDataTableTitle
                    title={props.title.clone()}
                    title_level={props.title_level}
                    id={props.id.clone()}
                    {onsearch}
                />
                <BBTable
                    titles={props.data.titles.clone()}
                    values={searched_values_state.deref().clone()}
                />
            </BBContainer>
        </BBContainer>
    }
}

pub type BBDataTableColumnTitle = AttrValue;
pub type BBDataTableValue = AttrValue;
pub type BBDataTableRow = HashMap<BBDataTableColumnTitle, BBDataTableValue>;
pub type BBDataTableValues = Vec<BBDataTableRow>;

#[derive(PartialEq)]
pub struct BBDataTableData {
    pub titles: Vec<BBDataTableColumnTitle>,
    pub values: Vec<HashMap<BBDataTableColumnTitle, BBDataTableValue>>,
}
