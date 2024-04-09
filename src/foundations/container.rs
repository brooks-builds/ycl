use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| BBContainerMargin::Normal)]
    pub margin: BBContainerMargin,
    #[prop_or_default]
    pub test_id: AttrValue,
}

#[function_component(BBContainer)]
pub fn component(props: &Props) -> Html {
    let container_class = match &props.margin {
        BBContainerMargin::None => None,
        BBContainerMargin::Fluid => Some("container-fluid"),
        BBContainerMargin::Normal => Some("container"),
    };

    let class = classes!(container_class, props.classes.clone());

    html! {
        <div {class} data-test-id={props.test_id.clone()}>
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq)]
pub enum BBContainerMargin {
    None,
    Fluid,
    Normal,
}
