use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    #[prop_or_default]
    pub checked: bool,
    pub id: AttrValue,
    pub value: AttrValue,
}

#[function_component(BBCheckbox)]
pub fn component(props: &Props) -> Html {
    html! {
        <>
            <input
                type="checkbox"
                name={props.name.clone()}
                id={props.id.clone()}
                checked={props.checked}
                class="mx-1"
                value={props.value.clone()}
             />
            <label for={props.id.clone()}>{props.label.clone()}</label>
        </>
    }
}
