use std::collections::HashMap;

use yew::prelude::*;

use crate::{
    components::data_table::{BBDataTable, BBDataTableData},
    elements::{
        table::BBTable,
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
    let mut values = vec![];

    let mut yew = HashMap::new();
    yew.insert("Tag Name".into(), "Yew".into());
    yew.insert("# Courses".into(), 10.to_string().into());
    let mut rust = HashMap::new();
    rust.insert("Tag Name".into(), "Rust".into());
    rust.insert("# Courses".into(), 1.to_string().into());
    let mut axum = HashMap::new();
    axum.insert("Tag Name".into(), "Axum".into());
    axum.insert("# Courses".into(), 0.to_string().into());

    values.push(yew);
    values.push(rust);
    values.push(axum);

    let data_table_data = BBDataTableData {
        titles: titles.clone(),
        values: values.clone(),
    };

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Tables"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Table"}</BBTitle>
            <BBTable
                {titles}
                {values}
            />
            <BBTitle level={BBTitleLevel::Two}>{"Data Table"}</BBTitle>
            <BBDataTable
                title="Course Articles"
                title_level={BBTitleLevel::Three}
                id="course-articles"
                data={data_table_data}
            />
        </BBContainer>
    }
}
