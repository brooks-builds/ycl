use std::collections::HashMap;

use gloo::console::log;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub titles: Vec<AttrValue>,
    pub rows: Vec<BBTableRow>,
    #[prop_or_default]
    pub row_slots: Vec<VNode>,
}

/// depracated
#[function_component(BBTable)]
pub fn component(props: &Props) -> Html {
    log!(format!("{:?}", &props.row_slots));
    html! {
        <table class="table">
          <thead>
            <tr>
                {
                    props.titles.iter().map(|title| html! {
                        <th scope="col">{title.clone()}</th>
                    }).collect::<Vec<Html>>()
                }
                {
                    if props.row_slots.len() > 0 {
                        Some(html! {
                            <th scope="col"></th>
                        })
                    } else {
                        None
                    }
                }
            </tr>
          </thead>
          <tbody>
            {
                props.rows.iter().enumerate().map(|(index, row)| html! {
                    <tr>
                        {
                            row.values.iter().map(|value| {log!(format!("value: {:?}", &value)); html! { <td>{value}</td>}}).collect::<Html>()
                        }
                        {
                            props.row_slots.get(index).cloned()
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
}
