use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub classes: Classes,
}

#[function_component(BBText)]
pub fn component(props: &Props) -> Html {
    let classes = classes!(props.classes.clone());

    html! {
        <p class={classes}>
            {props.children.clone()}
        </p>
    }
}
