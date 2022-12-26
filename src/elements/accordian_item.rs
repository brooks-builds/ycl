use ::yew::prelude::*;

use super::title::{BBTitle, BBTitleLevel};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub title_level: BBTitleLevel,
    pub children: Children,
    pub id: Option<String>,
}

#[function_component(BBAccordianItem)]
pub fn component(props: &Props) -> Html {
    html! {
        <div class="accordion-item">
            <BBTitle classes={classes!("accordion-header")} id={props.id.clone()} level={props.title_level}>
                <button class="accordion-button" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
                    {props.title.clone()}
                </button>
            </BBTitle>
            <div id="collapseOne" class="accordion-collapse collapse show" aria-labelledby="headingOne" data-bs-parent="#accordionExample">
                <div class="accordion-body">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}
