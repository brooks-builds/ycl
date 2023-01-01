use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BBRow)]
pub fn component(props: &Props) -> Html {
    html! {
        <div class="row">
            {props.children.clone()}
        </div>
    }
}
