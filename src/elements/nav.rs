use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| "navigation".into())]
    pub aria_label: AttrValue,
}

#[function_component(BBNav)]
pub fn component(props: &Props) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg navbar-light bg-light" aria-label={props.aria_label.clone()}>
            {props.children.clone()}
        </nav>
    }
}
