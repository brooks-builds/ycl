use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {}

#[function_component]
pub fn BBList(props: &Props) -> Html {
    html! {
        <h1>{"List"}</h1>
    }
}
