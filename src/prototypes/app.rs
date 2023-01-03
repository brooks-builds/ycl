use super::router::{switch, Route};
use crate::foundations::container::BBContainer;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </BBContainer>
    }
}
