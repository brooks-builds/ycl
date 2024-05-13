use yew::prelude::*;

use crate::components::dual_list::BBDualList;

#[function_component()]
pub fn DualList() -> Html {
    html! {
        <BBDualList title="Dual List" />
    }
}
