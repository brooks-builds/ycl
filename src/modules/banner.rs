use crate::{
    elements::icon::{BBIcon, BBIconSize, BBIconType},
    foundations::{align_text::AlignText, container::BBContainer},
};
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    pub banner_type: BBBannerType,
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
            { props.banner_type.icon() }
            {props.text.clone()}
            <button class="btn-close" type="button" data-bs-dismiss="alert" aria-label="Close"></button>
        </div>
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBBannerType {
    Error,
    Discord,
}

impl BBBannerType {
    pub fn class(&self) -> Classes {
        match self {
            Self::Error => classes!("alert-danger"),
            Self::Discord => classes!("alert-info"),
        }
    }

    pub fn icon(&self) -> VNode {
        match self {
            Self::Error => {
                html! { <BBIcon icon_type={BBIconType::Warning} size={BBIconSize::Tiny} /> }
            }
            Self::Discord => {
                html! { <BBIcon icon_type={BBIconType::Discord} size={BBIconSize::Tiny} /> }
            }
        }
    }
}
