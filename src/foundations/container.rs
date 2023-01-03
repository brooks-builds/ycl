use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| BBContainerMargin::None)]
    pub margin: BBContainerMargin,
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
        <section {class}>
            {props.children.clone()}
        </section>
    }
}

#[derive(PartialEq)]
pub enum BBContainerMargin {
    None,
    Fluid,
    Normal,
}
