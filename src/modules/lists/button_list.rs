use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<BBButtonListItem>,
    pub onclick: Callback<AttrValue>,
}

#[function_component(BBButtonList)]
pub fn component(props: &Props) -> Html {
    let props_onclick = props.onclick.clone();

    let button_onclick = Callback::from(move |event: MouseEvent| {
        let target = event.target().unwrap();
        let button_target = target.unchecked_into::<HtmlButtonElement>();
        let id = button_target.id();
        props_onclick.emit(AttrValue::from(id));
    });

    html! {
        <div class="list-group">
            {
                props.items.iter().map(|item| html! {
                  <button type="button" class="list-group-item list-group-item-action" aria-current="true" onclick={button_onclick.clone()} id={item.id.clone()}>
                    {item.label.clone()}
                  </button>
                }).collect::<Vec<Html>>()
            }
        </div>
    }
}

#[derive(PartialEq, Clone)]
pub struct BBButtonListItem {
    pub label: AttrValue,
    pub id: AttrValue,
}
