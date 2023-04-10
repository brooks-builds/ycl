pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<BBButtonListItem>,
}

#[function_component(BBButtonList)]
pub fn component(props: &Props) -> Html {
    html! {
        <div class="list-group">
            {
                props.items.iter().map(|item| html! {
                  <button type="button" class="list-group-item list-group-item-action" aria-current="true">
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
}
