use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub description: AttrValue,
    pub media: Children,
    pub title_level: BBTitleLevel,
}

#[function_component(BBLmsPromo)]
pub fn component(props: &Props) -> Html {
    html! {
      <div class="px-4 py-5 my-5 text-center">
        <BBTitle
            classes="display-5 fw-bold"
            level={props.title_level.clone()}
            align={AlignText::Center}
        >
            {props.title.clone()}
        </BBTitle>
        <div class="col-lg-6 mx-auto">
          <p class="lead mb-4">{props.description.clone()}</p>
          <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            {
                props.media.clone()
            }
          </div>
        </div>
      </div>
    }
}
