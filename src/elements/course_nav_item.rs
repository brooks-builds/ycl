use crate::elements::icon::{BBIcon, BBIconSize, BBIconType};
use crate::elements::tooltip::BBTooltip;
use crate::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle};
use stylist::{css, yew::styled_component, Style};
use yew::prelude::*;
use yew_router::Routable;

#[derive(PartialEq, Properties)]
pub struct Props<R>
where
    R: Routable + 'static,
{
    pub completed: bool,
    pub title: AttrValue,
    pub current: bool,
    pub children: Option<Vec<BBCourseNavArticle<R>>>,
    pub state: BBCourseNavItemState,
}

#[styled_component(BBCourseNavItem)]
pub fn component<R: Routable + 'static>(props: &Props<R>) -> Html {
    let class = classes!(
        "list-group-item",
        if props.current { Some("active") } else { None },
        props.state.css(),
    );

    html! {
        <BBTooltip title={props.state.title()}>
            <li {class}>
                {
                    if props.completed {
                        html! { <BBIcon icon_type={BBIconType::Check} size={BBIconSize::Tiny} /> }
                    } else {
                        html! { <span class={classes!(BBIconSize::Tiny.css(), Style::new(css!("display: inline-block;")).unwrap())}></span>}
                    }
                }
                {&props.title}
                {
                    props.children.clone().map(|articles| {
                        html! { <BBCourseNav<R> {articles} classes={classes!("ms-4")} />}
                    })
                }
            </li>
        </BBTooltip>
    }
}

#[derive(PartialEq, Clone)]
pub enum BBCourseNavItemState {
    Available,
    NotYetCreated,
    NeedToPurchase,
}

impl BBCourseNavItemState {
    pub fn css(&self) -> Option<&'static str> {
        match self {
            Self::NotYetCreated | Self::NeedToPurchase => Some("disabled"),
            _ => None,
        }
    }

    pub fn title(&self) -> Option<&'static str> {
        match self {
            Self::NotYetCreated => Some("This article is not available yet"),
            Self::NeedToPurchase => Some("Purchase this article to read"),
            _ => None,
        }
    }
}
