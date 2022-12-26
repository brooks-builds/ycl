use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BBAccordian)]
pub fn component(props: &Props) -> Html {
    html! {
        <div class="accordion">
            {
                for props.children.iter()
            }
        </div>
    }
}
