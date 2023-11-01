use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    #[prop_or_default]
    pub checked: bool,
    pub id: AttrValue,
    pub value: AttrValue,
    #[prop_or_default]
    pub onchecked: Callback<()>,
}

#[function_component(BBCheckbox)]
pub fn component(props: &Props) -> Html {
    let onchecked = {
        let props_onchecked = props.onchecked.clone();

        Callback::from(move |event| {
            props_onchecked.emit(());
            log!(event);
        })
    };

    html! {
        <>
            <input
                type="checkbox"
                name={props.name.clone()}
                id={props.id.clone()}
                checked={props.checked}
                class="mx-1"
                value={props.value.clone()}
                oninput={onchecked}
             />
            <label for={props.id.clone()}>{props.label.clone()}</label>
        </>
    }
}
