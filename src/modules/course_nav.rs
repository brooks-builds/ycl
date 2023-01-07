use yew::prelude::*;
use stylist::{yew::styled_component};
use crate::elements::course_nav_item::{BBCourseNavItem, BBCourseNavItemState};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub articles: Vec<BBCourseNavArticle>,
    #[prop_or_default]
    pub classes: Classes
}

#[styled_component(BBCourseNav)]
pub fn component(props: &Props) -> Html {
    let class = classes!(
        props.classes.clone(),
        "list-group",
    );

    html! {
        <ul {class}>
            {
                props
                    .articles
                    .clone()
                    .into_iter()
                    .map(|article| {
                        html! {
                            <BBCourseNavItem
                                completed={article.completed}
                                title={article.title}
                                current={article.current}
                                children={article.children}
                                state={article.state} />
                        }
                    })
                    .collect::<Html>()
            }
        </ul>
    }
}

#[derive(PartialEq, Clone)]
pub struct BBCourseNavArticle {
    pub completed: bool,
    pub title: AttrValue,
    pub current: bool,
    pub children: Option<Vec<BBCourseNavArticle>>,
    pub state: BBCourseNavItemState,
}

impl BBCourseNavArticle {
    pub fn active_class(&self) -> Option<&'static str> {
        if self.current {
            Some("active")
        } else {
            None
        }
    }
}
