use yew::prelude::*;

use crate::elements::title::{BBTitle, BBTitleLevel};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHeaders)]
pub fn component(_props: &Props) -> Html {
    html! {
        <main>
            <BBTitle level={BBTitleLevel::One}>{"Headers"}</BBTitle>
        </main>
    }
}
