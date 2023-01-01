use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(BBRow)]
pub fn component(props: &Props) -> Html {
    let class = classes!(props.classes.clone(), "row");

    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}
