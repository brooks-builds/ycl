use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default()]
    pub children: Children,
}

#[function_component(BBForm)]
pub fn component(props: &Props) -> Html {
    html! {
        <form>{props.children.clone()}</form>
    }
}
