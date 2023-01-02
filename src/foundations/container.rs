use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(BBContainer)]
pub fn component(props: &Props) -> Html {
    let container_class = if props.full_width {
        None
    } else {
        Some("container")
    };

    let class = classes!(container_class, props.classes.clone());

    html! {
        <section {class}>
            {props.children.clone()}
        </section>
    }
}
