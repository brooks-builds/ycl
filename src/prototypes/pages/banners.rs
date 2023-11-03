use crate::{
    elements::{
        icon::BBIconType,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{align_text::AlignText, container::BBContainer},
    modules::banner::{BBBanner, BBBannerType},
};
use yew::prelude::*;

#[function_component(Banners)]
pub fn component() -> Html {
    let onclick = Callback::from(|_| {
        gloo::console::info!("banner was clicked");
    });

    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Banners"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Discord Banner"}</BBTitle>
            <BBBanner banner_type={BBBannerType::CallToAction} text="Join the Community" icon={BBIconType::Discord} {onclick} />

            <BBTitle level={BBTitleLevel::Two}>{"Warning Banner"}</BBTitle>
            <BBBanner banner_type={BBBannerType::Error} text="There was a problem getting your course" icon={BBIconType::Warning} />

            <BBTitle level={BBTitleLevel::Two}>{"Success Banner"}</BBTitle>
            <BBBanner banner_type={BBBannerType::Success} text="Account created, you can now log in" icon={BBIconType::Heart} />
        </BBContainer>
    }
}
