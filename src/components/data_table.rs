#![allow(non_camel_case_types)]
use std::ops::Deref;

use crate::{
    components::data_table_title::BBDataTableTitle,
    elements::{
        table::{BBTable, BBTableDrop, BBTableRow},
        title::BBTitleLevel,
    },
    foundations::container::BBContainer,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_else(|| BBTitleLevel::Two)]
    pub title_level: BBTitleLevel,
    pub id: AttrValue,
    pub rows: Vec<BBTableRow>,
    pub titles: Vec<AttrValue>,
    #[prop_or_default]
    pub drag: bool,
    #[prop_or_default]
    pub ondrop: Callback<BBTableDrop>,
}

#[function_component(BBDataTable)]
pub fn component(props: &Props) -> Html {
    let searched_rows_state = use_state(move || Vec::new());

    {
        let searched_rows_state = searched_rows_state.clone();

        use_effect_with(props.rows.clone(), move |props_rows| {
            searched_rows_state.set(props_rows.to_owned());
            || {}
        });
    }

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
                    drag={props.drag}
                    ondrop={props.ondrop.clone()}
                />
            </BBContainer>
        </BBContainer>
    }
}
