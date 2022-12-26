use ::yew::prelude::*;

use super::title::BBTitleLevel;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub title_level: BBTitleLevel,
    pub children: Children,
    pub id: String,
    pub parent_id: String,
}

#[function_component(BBAccordianItem)]
pub fn component(props: &Props) -> Html {
    html! {
    <>
        <div class="accordion-item">
            <div class="accordion-header" id={props.id.clone()}>
            <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target={format!("#{}", &props.id)} aria-expanded="false" aria-controls="collapseTwo">
                {props.title.clone()}
            </button>
            </div>
            <div id={props.id.clone()} class="accordion-collapse collapse" aria-labelledby={props.id.clone()} data-bs-parent={props.parent_id.clone()}>
            <div class="accordion-body">
                {props.children.clone()}
            </div>
            </div>
        </div>
    </>
    }
}
