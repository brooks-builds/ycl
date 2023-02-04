use crate::{
    elements::icon::{BBIcon, BBIconSize, BBIconType},
    foundations::align_text::AlignText,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    pub banner_type: BBBannerType,
    pub icon: BBIconType,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(BBBanner)]
pub fn component(props: &Props) -> Html {
    let class = classes!(
        "alert",
        props.banner_type.class(),
        "alert-dismissible",
        "show",
        "fade",
        AlignText::Center.class()
    );

    html! {
        <div {class} role="alert">
            <span onclick={props.onclick.clone()}>
                <BBIcon icon_type={props.icon} size={BBIconSize::Tiny} />
                {props.text.clone()}
            </span>
            <button class="btn-close" type="button" data-bs-dismiss="alert" aria-label="Close"></button>
        </div>
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBBannerType {
    Error,
    CallToAction,
}

impl BBBannerType {
    pub fn class(&self) -> Classes {
        match self {
            Self::Error => classes!("alert-danger"),
            Self::CallToAction => classes!("alert-info"),
        }
    }
}
