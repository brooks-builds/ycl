use yew::prelude::*;

use crate::elements::text::BBText;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub label: AttrValue,
    pub percentage: AttrValue,
}

#[function_component(BBProgressBar)]
pub fn component(props: &Props) -> Html {
    html! {
        <div>
              <BBText>{props.label.clone()}</BBText>
            <div class="progress" role="progressbar" aria-label={props.label.clone()} aria-valuenow={props.percentage.clone()} aria-valuemin="0" aria-valuemax="100">
              <div class="progress-bar" style={format!("width: {}%", props.percentage.clone())}></div>
            </div>
        </div>
    }
}
