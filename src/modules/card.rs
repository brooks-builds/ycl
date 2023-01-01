use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::elements::{
    text::BBText,
    title::{BBTitle, BBTitleLevel},
};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub title_level: BBTitleLevel,
    pub title: AttrValue,
    pub text: Option<AttrValue>,
    pub internal_link: Option<T>,
}

#[styled_component(BBCard)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let class = style!(
        r#"
        width: 18rem;
    "#
    )
    .unwrap();

    html! {
        <div class={classes!("card", class, "mx-1", "my-1")}>
            <div class="card-body">
                {
                    if props.internal_link.is_some() {
                        html! {
                            <Link<T> to={props.internal_link.clone().unwrap()}>
                                <BBTitle level={props.title_level} classes={classes!("card-title")}>{props.title.clone()}</BBTitle>
                            </Link<T>>
                        }
                    } else {
                        html! {
                            <BBTitle level={props.title_level} classes={classes!("card-title")}>{props.title.clone()}</BBTitle>
                        }
                    }
                }
                <BBText classes="card-text">{props.text.clone()}</BBText>
            </div>
        </div>
    }
}
