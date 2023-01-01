use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BBContainer)]
pub fn component(props: &Props) -> Html {
    html! {
        <section class="container">
            {props.children.clone()}
        </section>
    }
}
