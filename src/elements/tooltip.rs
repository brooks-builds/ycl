use yew::{prelude::*};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: Option<AttrValue>,
    pub children: Children,
}

#[function_component(BBTooltip)]
pub fn component(props: &Props) -> Html {
    if let Some(title) = props.title.clone() {
        html! {
            <span data-bs-toggle="tooltip" data-bs-title={title}>
                {props.children.clone()}
            </span>
        }
    } else {
        html! { <>{props.children.clone()}</> }
    }
}
