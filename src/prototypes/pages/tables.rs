use std::collections::HashMap;

use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    components::data_table::BBDataTable,
    elements::{
        table::{BBTable, BBTableRow},
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        container::{BBContainer, BBContainerMargin},
    },
};

#[function_component(TablesPrototype)]
pub fn component() -> Html {
    let titles = vec!["Tag Name".into(), "# Courses".into()];
    let data_table_titles = vec!["Tag Name".into(), "# Courses".into(), "preview".into()];
    let rows = vec![
        BBTableRow {
            id: "1".into(),
            values: vec!["Rust".into(), "10".into()],
        },
        BBTableRow {
            id: "2".into(),
            values: vec!["Axum".into(), "1".into()],
        },
        BBTableRow {
            id: "3".into(),
            values: vec!["SQLx".into(), "2".into()],
        },
    ];

    let preview_oninput = Callback::from(|event: InputEvent| {
        let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        let value = input_element.value();
        let checked = input_element.checked();
        log!(value, checked);
    });

    let row_slots = {
        let preview_oninput = preview_oninput.clone();

        rows.iter()
            .map(move |row| {
                html! {
                    <input type="checkbox" value={row.id.clone()} oninput={preview_oninput.clone()}/>
                }
            })
            .collect::<Vec<Html>>()
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Tables"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Table"}</BBTitle>
            <BBTable
                titles={titles.clone()}
                rows={rows.clone()}
            />
            <BBTitle level={BBTitleLevel::Two}>{"Data Table"}</BBTitle>
            <BBDataTable
                title="Course Articles"
                title_level={BBTitleLevel::Three}
                id="course-articles"
                {rows}
                titles={data_table_titles}
                {row_slots}
            />
        </BBContainer>
    }
}
