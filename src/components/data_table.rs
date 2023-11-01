#![allow(non_camel_case_types)]
use std::{collections::HashMap, ops::Deref};

use crate::{
    components::data_table_title::BBDataTableTitle,
    elements::{
        checkbox::BBCheckbox,
        table::{BBTable, BBTableRow},
        title::BBTitleLevel,
    },
    foundations::container::BBContainer,
};
use gloo::console::log;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_else(|| BBTitleLevel::Two)]
    pub title_level: BBTitleLevel,
    pub id: AttrValue,
    pub rows: Vec<BBTableRow>,
    pub titles: Vec<AttrValue>,
    #[prop_or_default()]
    pub row_slots: Vec<VNode>,
}

#[function_component(BBDataTable)]
pub fn component(props: &Props) -> Html {
    let searched_rows_state = {
        let props_rows = props.rows.clone();

        use_state(move || props_rows)
    };

    let onsearch = {
        let searched_rows_state = searched_rows_state.clone();
        let props_rows = props.rows.clone();

        Callback::from(move |search_text: AttrValue| {
            let searched_rows = props_rows
                .clone()
                .into_iter()
                .filter(|prop_row| {
                    for value in prop_row.values.iter() {
                        if value.as_str().to_lowercase().contains(search_text.as_str()) {
                            return true;
                        }
                    }

                    return false;
                })
                .collect::<Vec<BBTableRow>>();

            searched_rows_state.set(searched_rows);
        })
    };

    let searched_values_checkboxes = searched_rows_state
        .iter()
        .map(|row| {
            html! {
                <input type="checkbox" id={row.id.clone()} />
            }
        })
        .collect::<Vec<Html>>();

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
                    titles={props.titles.clone()}
                    rows={searched_rows_state.deref().clone()}
                    row_slots={props.row_slots.clone()}
                />
            </BBContainer>
        </BBContainer>
    }
}
