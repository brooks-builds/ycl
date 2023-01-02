use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
    modules::site_footer::BBSiteFooter,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PFooters)]
pub fn component(_props: &Props) -> Html {
    html! {
        <main>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Footers"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Main site footer"}</BBTitle>
            <BBSiteFooter />
        </main>
    }
}
