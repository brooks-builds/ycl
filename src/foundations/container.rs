use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub full_width: bool,
}

#[function_component(BBContainer)]
pub fn component(props: &Props) -> Html {
    let class = if props.full_width {
        None
    } else {
        Some("container")
    };

    html! {
        <section {class}>
            {props.children.clone()}
        </section>
    }
}
