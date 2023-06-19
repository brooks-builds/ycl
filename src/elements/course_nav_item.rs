use crate::elements::icon::{BBIcon, BBIconSize, BBIconType};
use crate::elements::route_or_not::BBRouteOrNot;
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
    #[prop_or_default]
    pub preview: bool,
    pub to: Option<R>,
    pub id: AttrValue,
}

#[styled_component(BBCourseNavItem)]
pub fn component<R: Routable + 'static>(props: &Props<R>) -> Html {
    let class = classes!(
        "list-group-item",
        if props.current { Some("active") } else { None },
    );
    let tooltip: AttrValue = if props.preview {
        "This article is available as a preview".into()
    } else {
        "Purchase this course to have access".into()
    };
    let title = if props.preview {
        format!("{} - (preview)", &props.title)
    } else {
        props.title.to_string()
    };

    html! {
        <li {class} data-id={props.id.clone()}>
            <BBRouteOrNot<R> to={props.to.clone()}>
                <BBTooltip title={tooltip}>
                    {
                        if props.completed {
                            html! { <BBIcon icon_type={BBIconType::Check} size={BBIconSize::Tiny} /> }
                        } else {
                            html! { <span class={classes!(BBIconSize::Tiny.css(), Style::new(css!("display: inline-block;")).unwrap())}></span>}
                        }
                    }
                    {&title}
                    {
                        props.children.clone().map(|articles| {
                            html! { <BBCourseNav<R> {articles} classes={classes!("ms-4")} />}
                        })
                    }
                </BBTooltip>
            </BBRouteOrNot<R>>
        </li>
    }
}
