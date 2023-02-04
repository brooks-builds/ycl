use yew::prelude::*;

use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::{align_text::AlignText, container::BBContainer},
    modules::banner::{BBBanner, BBBannerType},
};

#[function_component(Banners)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Banners"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Discord Banner"}</BBTitle>
            <BBBanner banner_type={BBBannerType::Discord} text="Join the Community" />

            <BBTitle level={BBTitleLevel::Two}>{"Warning Banner"}</BBTitle>
            <BBBanner banner_type={BBBannerType::Error} text="There was a problem getting your course" />
        </BBContainer>
    }
}
