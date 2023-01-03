use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: AttrValue,
    pub alt: AttrValue,
}

#[function_component(BBImage)]
pub fn component(props: &Props) -> Html {
    html! {
        <img
            src={props.src.clone()}
            alt={props.alt.clone()}
        />
    }
}
