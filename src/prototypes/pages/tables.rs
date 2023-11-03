use gloo::console::info;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    components::data_table::BBDataTable,
    elements::{
        table::{BBTable, BBTableDrop, BBTableRow},
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
    let preview_oninput = Callback::from(|event: InputEvent| {
        let input_element = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        let value = input_element.value();
        let checked = input_element.checked();
        info!(value, checked);
    });

    let rows = vec![
        BBTableRow {
            id: "1".into(),
            values: vec!["Rust".into(), "10".into()],
            slot: Some(
                html! { <input type="checkbox" value="1" oninput={preview_oninput.clone()} /> },
            ),
        },
        BBTableRow {
            id: "2".into(),
            values: vec!["Axum".into(), "1".into()],
            slot: Some(
                html! { <input type="checkbox" value="2" oninput={preview_oninput.clone()} /> },
            ),
        },
        BBTableRow {
            id: "3".into(),
            values: vec!["SQLx".into(), "2".into()],
            slot: Some(
                html! { <input type="checkbox" value="3" oninput={preview_oninput.clone()} /> },
            ),
        },
    ];

    let ondrop = Callback::from(|drop_data: BBTableDrop| {
        info!("table row dropped:", format!("{drop_data:?}"))
    });

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
                drag={true}
                {ondrop}
            />
        </BBContainer>
    }
}
