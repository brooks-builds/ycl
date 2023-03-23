use std::collections::HashMap;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub titles: Vec<AttrValue>,
    pub values: Vec<HashMap<AttrValue, AttrValue>>,
}

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
                props.values.iter().map(|value_map| html! {
                    <tr>
                        {
                            props.titles.iter().map(|title| {
                                let value = value_map.get(title);
                                html! { <td>{value}</td> }
                            }).collect::<Vec<Html>>()
                        }
                    </tr>
                }).collect::<Vec<Html>>()
            }
          </tbody>
        </table>
    }
}

#[derive(PartialEq)]
pub struct BBTableValue {
    pub title: AttrValue,
    pub values: Vec<AttrValue>,
}
