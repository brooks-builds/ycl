use gloo::console::error;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub titles: Vec<AttrValue>,
    pub rows: Vec<BBTableRow>,
    #[prop_or_default]
    pub drag: bool,
    #[prop_or_default]
    pub ondrop: Callback<BBTableDrop>,
}

/// depracated
#[function_component(BBTable)]
pub fn component(props: &Props) -> Html {
    let ondragstart = {
        Callback::from(|event: DragEvent| {
            let Some(data) = event.data_transfer() else {
                error!("could not get data transfer when starting to drag");
                return;
            };
            let Some(target) = event.target() else {
                error!("no target when starting to drag");
                return;
            };
            let target_element = target.unchecked_into::<HtmlElement>();
            let Some(data_id) = target_element.get_attribute("data-id") else {
                error!("target element does not have data-id set when starting to drag");
                return;
            };

            if let Err(error) = data.set_data("text/plain", &data_id) {
                error!("Error setting data when starting to drag", error);
            };
        })
    };

    let ondragover = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();

            let Some(target) = event.target() else {
                error!("missing target when dragging over");
                return;
            };
            let target_element = target.unchecked_into::<HtmlElement>();
            let Some(target_parent) = target_element.parent_element() else {
                error!("target element doesn't have a parent when dragging over");
                return;
            };
            let Some(target_parent_id) = target_parent.get_attribute("data-id") else {
                error!("Element we're scrolling over doesn't have a data-id attribute");
                return;
            };
            let id = if let Some(data_transfer) = event.data_transfer() {
                match data_transfer.get_data("text/plain") {
                    Ok(id) => id,
                    Err(error) => {
                        error!("Could not extract id from data transfer: ", error);
                        return;
                    }
                }
            } else {
                error!("Attempting to get a data transfer opject when it doesn't exist");
                return;
            };

            if target_parent_id != id {
                if let Err(error) = target_parent.class_list().add_1("drop_target") {
                    error!(
                        "Error adding drop target to element while dragging over",
                        error
                    );
                    return;
                };
            }
        })
    };

    let ondragleave = Callback::from(|event: DragEvent| {
        let Some(target) = event.target() else {
            error!("could not find target element when leaving while dragging");
            return;
        };
        let target_element = target.unchecked_into::<Element>();
        let Some(target_parent) = target_element.parent_element() else {
            error!("target element doesn't have a parent when dragging over");
            return;
        };

        if let Err(error) = target_parent.class_list().remove_1("drop_target") {
            error!(
                "Encountered an error when attempting to remove drop target class from element:",
                error
            );
            return;
        }
    });

    let ondrop = {
        let props_ondrop = props.ondrop.clone();

        Callback::from(move |event: DragEvent| {
            let Some(target) = event.target() else {
                error!("could not find target element when leaving while dragging");
                return;
            };
            let target_element = target.unchecked_into::<Element>();
            let Some(target_parent) = target_element.parent_element() else {
                error!("target element doesn't have a parent when dragging over");
                return;
            };
            let Some(parent_id) = target_parent.get_attribute("data-id") else {
                error!("Could not find data-id when dropping");
                return;
            };
            let id = if let Some(data_transfer) = event.data_transfer() {
                match data_transfer.get_data("text/plain") {
                    Ok(id) => id,
                    Err(error) => {
                        error!("Could not extract id from data transfer:", error);
                        return;
                    }
                }
            } else {
                error!("Attempting to get a data transfer opject when it doesn't exist");
                return;
            };

            if let Err(error) = target_parent.class_list().remove_1("drop_target") {
                error!(
                "Encountered an error when attempting to remove drop target class from element:",
                error
            );
                return;
            }

            props_ondrop.emit(BBTableDrop {
                id: id.into(),
                dropped_on_id: parent_id.into(),
            });
        })
    };

    html! {
        <table class="table">
          <thead>
            <tr>
                {
                    props.titles.iter().map(|title| html! {
                        <th scope="col">{title.clone()}</th>
                    }).collect::<Vec<Html>>()
                }
            </tr>
          </thead>
          <tbody>
            {
                props.rows.iter().map(move |row| html! {
                    <tr draggable={"true"} ondragstart={ondragstart.clone()} ondragover={ondragover.clone()} data-id={row.id.clone()} ondragleave={ondragleave.clone()} ondrop={ondrop.clone()}>
                        {
                            row.values.iter().map(|value| {html! { <td>{value}</td>}}).collect::<Html>()
                        }
                        {
                            row.slot.clone()
                        }
                    </tr>
                }).collect::<Vec<Html>>()
            }
          </tbody>
        </table>
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BBTableRow {
    pub id: AttrValue,
    pub values: Vec<AttrValue>,
    pub slot: Option<VNode>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BBTableDrop {
    pub id: AttrValue,
    pub dropped_on_id: AttrValue,
}
