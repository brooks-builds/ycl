use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default()]
    pub children: Children,
    #[prop_or_default]
    pub onsubmit: Callback<()>,
}

#[function_component(BBForm)]
pub fn component(props: &Props) -> Html {
    let onsubmit = {
        let prop_onsubmit = props.onsubmit.clone();

        Callback::from(move |event: SubmitEvent| {
            gloo::console::log!("form submitted");
            event.prevent_default();
            prop_onsubmit.emit(());
        })
    };

    html! {
        <form onsubmit={onsubmit} method="GET" action="/">{props.children.clone()}</form>
    }
}
