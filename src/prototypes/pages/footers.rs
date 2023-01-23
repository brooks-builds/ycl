use yew::prelude::*;

use crate::{elements::title::{BBTitle, BBTitleLevel}, foundations::align_text::AlignText};
use crate::modules::site_footer::BBSiteFooter;
use crate::foundations::container::BBContainer;

#[function_component(Footers)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Footers"}</BBTitle>
            <BBSiteFooter />
        </BBContainer>
    }
}
