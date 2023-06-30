use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    pub test_id: Option<AttrValue>,
}

#[function_component(BBRow)]
pub fn component(props: &Props) -> Html {
    let class = classes!(props.classes.clone(), "row");

    html! {
        <div {class} data-testid={props.test_id.clone()}>
            {props.children.clone()}
        </div>
    }
}
