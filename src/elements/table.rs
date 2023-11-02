use std::collections::HashMap;

use gloo::console::{error, log};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::{prelude::*, virtual_dom::VNode};

use crate::elements::icon::{BBIcon, BBIconSize, BBIconType};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub titles: Vec<AttrValue>,
    pub rows: Vec<BBTableRow>,
    #[prop_or_default]
    pub drag: bool,
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
            if let Err(error) = target_element.class_list().add_1("drop_target") {
                error!(
                    "Error adding drop target to element while dragging over",
                    error
                );
                return;
            };
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
                    <tr draggable={"true"} ondragstart={ondragstart.clone()} ondragover={ondragover.clone()} data-id={row.id.clone()}>
                        {
                            row.values.iter().map(|value| {log!(format!("value: {:?}", &value)); html! { <td>{value}</td>}}).collect::<Html>()
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
