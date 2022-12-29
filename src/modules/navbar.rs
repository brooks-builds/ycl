use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub brand: Children,
    pub center: Children,
    pub right: Children,
}

#[function_component(BBNavbar)]
pub fn component(props: &Props) -> Html {
    html! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                {props.brand.clone()}
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <div class="mx-auto"></div>
                    {props.center.clone()}
                    <div class="mx-auto"></div>
                    {props.right.clone()}
                </div>
            </div>
        </nav>
    }
}
