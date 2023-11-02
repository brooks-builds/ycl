use std::collections::HashMap;

use gloo::console::log;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub titles: Vec<AttrValue>,
    pub rows: Vec<BBTableRow>,
}

/// depracated
#[function_component(BBTable)]
pub fn component(props: &Props) -> Html {
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
                props.rows.iter().map(|row| html! {
                    <tr>
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
