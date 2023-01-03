use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BBNav)]
pub fn component(props: &Props) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light">
            {props.children.clone()}
        </nav>
    }
}
