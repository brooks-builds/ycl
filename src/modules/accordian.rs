use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub id: Option<String>,
}

#[function_component(BBAccordian)]
pub fn component(props: &Props) -> Html {
    html! {
        <div class="accordion" id={props.id.clone()}>
            {
                for props.children.iter()
            }
        </div>
    }
}
