use yew::prelude::*;

use crate::elements::title::BBTitleLevel;

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
        <h1 class="display-5 fw-bold">{props.title.clone()}</h1>
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
