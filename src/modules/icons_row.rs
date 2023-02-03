use crate::elements::icon::{BBIcon, BBIconSize, BBIconType};
use crate::foundations::column::BBCol;
use crate::foundations::row::BBRow;
use stylist::yew::styled_component;
use stylist::{css, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub icons: Vec<BBIconsRowList>,
}

#[styled_component(BBIconsRow)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBRow classes="text-center pb-3">
            <BBCol>
                {
                    props.icons.clone().into_iter().map(|icon| {
                        html! {
                            <a href={icon.href} >
                                <BBIcon icon_type={icon.icon_type} size={BBIconSize::Small} />
                            </a>
                        }
                    }).collect::<Vec<Html>>()
                }
            </BBCol>
       </BBRow>
    }
}

#[derive(PartialEq, Clone)]
pub struct BBIconsRowList {
    pub icon_type: BBIconType,
    pub href: AttrValue,
}

impl BBIconsRowList {
    pub fn new(icon_type: BBIconType, href: impl Into<AttrValue>) -> Self {
        Self {
            icon_type,
            href: href.into(),
        }
    }
}
