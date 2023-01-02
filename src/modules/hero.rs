use stylist::{css, style, Style};
use yew::prelude::*;

use crate::{
    elements::{
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{
        align_text::AlignText,
        column::{BBCol, BBColWidth},
        row::BBRow,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: AttrValue,
    pub text: AttrValue,
    #[prop_or_else(|| BBTitleLevel::One)]
    pub title_level: BBTitleLevel,
    #[prop_or_default]
    pub main: Children,
    pub img: Option<AttrValue>,
}

#[function_component(BBHero)]
pub fn component(props: &Props) -> Html {
    let (main_width, css) = if let Some(img) = props.img.clone() {
        let class = style!("background-image: url(\"${img}\");", img = img).unwrap();
        (BBColWidth::Seven, Some(class))
    } else {
        (BBColWidth::None, None)
    };

    html! {
        <BBRow classes="px-4 my-5 bg-secondary">
            {
                if props.img.is_some() {
                    Some(html! {
                        <BBCol width={BBColWidth::Five} classes={classes!(css)}>
                            <div></div>
                        </BBCol>
                    })
                } else {
                    None
                }
            }
           <BBCol width={main_width} classes="py-5">
                <div>
                    <BBTitle level={props.title_level} align={AlignText::Center}>{props.title.clone()}</BBTitle>
                    <BBText align={AlignText::Center}>{props.text.clone()}</BBText>
                    {props.main.clone()}
                </div>
           </BBCol>
        </BBRow>
    }
}
