use stylist::{style, yew::styled_component};
use yew::prelude::*;

use crate::elements::{
    text::BBText,
    title::{BBTitle, BBTitleLevel},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title_level: BBTitleLevel,
    pub title: AttrValue,
    pub text: Option<AttrValue>,
}

#[styled_component(BBCard)]
pub fn component(props: &Props) -> Html {
    let class = style!(
        r#"
        width: 18rem;
    "#
    )
    .unwrap();

    html! {
        <div class={classes!("card", class, "mx-1", "my-1")}>
            <div class="card-body">
                <BBTitle level={props.title_level} classes={classes!("card-title")}>{props.title.clone()}</BBTitle>
                <BBText classes="card-text">{props.text.clone()}</BBText>
            </div>
        </div>
    }
}
