use yew::prelude::*;

use crate::{
    components::list::BBList,
    elements::title::BBTitleLevel,
    foundations::{column::BBCol, container::BBContainer, row::BBRow},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub available_title: AttrValue,
    pub selected_title: AttrValue,
    pub title_level: BBTitleLevel,
    pub id: AttrValue,
}

#[function_component]
pub fn BBDualList(props: &Props) -> Html {
    let available_id = format!("{}-available", props.id.as_str());
    let selected_id = format!("{}-selected", props.id.as_str());

    html! {
        <BBRow>
            <BBCol>
                <BBList title={props.available_title.clone()} title_level={props.title_level.clone()} border={true} id={available_id} />
            </BBCol>
            <BBCol>
            </BBCol>
            <BBCol>
                <BBList title={props.selected_title.clone()} title_level={props.title_level.clone()} border={true} id={selected_id} />
            </BBCol>
        </BBRow>
    }
}
