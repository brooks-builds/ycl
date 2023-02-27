use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BBForm)]
pub fn component(props: &Props) -> Html {
    html! {
        <form>{props.children.clone()}</form>
    }
}
