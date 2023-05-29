#![allow(non_camel_case_types)]

use yew::prelude::*;

use crate::{
    elements::{
        button::BBButton,
        icon::{BBIcon, BBIconSize, BBIconType},
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{column::BBCol, row::BBRow},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icon: Option<BBIconType>,
    pub title: AttrValue,
    pub title_level: BBTitleLevel,
    #[prop_or_else(|| AttrValue::from("Show All"))]
    pub action: AttrValue,
    #[prop_or_default]
    pub on_action: Callback<()>,
    #[prop_or_default]
    pub more: bool,
}

#[function_component(BBSectionHeader)]
pub fn component(props: &Props) -> Html {
    html! {
        <header>
            <BBRow classes="align-items-center">
                <BBCol>
                    <BBTitle level={props.title_level}>
                        {
                            if props.icon.is_some() {
                                Some(html! {
                                    <BBIcon icon_type={props.icon.unwrap()} size={BBIconSize::Small} />
                                })
                            } else {
                                None
                            }
                        }
                        {props.title.clone()}
                    </BBTitle>
                </BBCol>
                <BBCol classes="align-right">
                    {
                        if props.more {
                            Some(html! {
                                <BBButton onclick={props.on_action.clone()}>{props.action.clone()}</BBButton>
                            })
                        } else {
                            None
                        }
                    }
                </BBCol>
            </BBRow>
        </header>
    }
}
