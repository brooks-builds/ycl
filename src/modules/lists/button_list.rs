use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlButtonElement};
pub use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Vec<BBButtonListItem>,
    pub onclick: Callback<AttrValue>,
    #[prop_or_default()]
    pub draggable: bool,
    #[prop_or_default()]
    pub ondrop: Callback<BBButtonListMove>,
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

    let ondragstart = Callback::from(|event: DragEvent| {
        let Some(target) = event.target() else {
            return;
        };
        let target = target.unchecked_into::<HtmlButtonElement>();
        let id = target.id();
        let Some(data) = event.data_transfer() else {
            gloo::console::error!("Error, data transfer object missing when dragging");
            return;
        };
        if let Err(error) = data.clear_data() {
            gloo::console::error!("Error clearing drag data:", error);
            return;
        };
        if let Err(error) = data.set_data("text/plain", &id) {
            gloo::console::error!("Error setting id on button:", error);
            return;
        };
    });

    let ondragenter = Callback::from(|event: DragEvent| {
        event.prevent_default();
    });

    let ondragover = {
        let draggable = props.draggable;

        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            if !draggable {
                return;
            }

            let Some(data) = event.data_transfer() else {
                gloo::console::error!(
                    "Error, data transfer object missing when dropping dragged object"
                );
                return;
            };

            let id = match data.get_data("text/plain") {
                Ok(data) => data,
                Err(error) => {
                    gloo::console::error!("Error getting drag drop data:", error);
                    return;
                }
            };

            let Some(target) = event.target() else {
                gloo::console::error!("target missing");
                return;
            };
            let target = target.unchecked_into::<Element>();
            let target_id = target.id();

            if id == target_id {
                return;
            }

            if let Err(error) = target.class_list().add_1("drop_target") {
                gloo::console::error!("Error setting drop target", error);
            }
        })
    };

    let ondragleave = Callback::from(|event: DragEvent| {
        let Some(target) = event.target() else {
            gloo::console::error!("target missing");
            return;
        };
        let target = target.unchecked_into::<Element>();
        if let Err(error) = target.class_list().remove_1("drop_target") {
            gloo::console::error!("Error removing drop target style", error);
        }
    });

    let ondrop = {
        let props_ondrop = props.ondrop.clone();

        Callback::from(move |event: DragEvent| {
            let Some(data) = event.data_transfer() else {
                gloo::console::error!(
                    "Error, data transfer object missing when dropping dragged object"
                );
                return;
            };

            let id = match data.get_data("text/plain") {
                Ok(data) => data,
                Err(error) => {
                    gloo::console::error!("Error getting drag drop data:", error);
                    return;
                }
            };

            let Some(target) = event.target() else {
                gloo::console::error!("target drop doesn't exist:");
                return;
            };

            let target = target.unchecked_into::<Element>();

            if let Err(error) = target.class_list().remove_1("drop_target") {
                gloo::console::error!("Error removing drop target style", error);
            }

            props_ondrop.emit(BBButtonListMove {
                moving_id: AttrValue::from(id),
                target_id: AttrValue::from(target.id()),
            });
        })
    };

    html! {
        <div
            class="list-group"
            ondragstart={ondragstart.clone()}
            {ondragenter}
            {ondragleave}
            {ondrop}
            {ondragover}
        >
            {
                props.items.iter().map(|item| html! {
                  <button
                    type="button"
                    class="list-group-item list-group-item-action btn"
                    aria-current="true"
                    onclick={button_onclick.clone()}
                    id={item.id.clone()}
                    draggable={props.draggable.to_string()}
                  >
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

#[derive(Debug)]
pub struct BBButtonListMove {
    pub moving_id: AttrValue,
    pub target_id: AttrValue,
}
