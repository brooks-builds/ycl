use crate::elements::course_nav_item::{BBCourseNavItem, BBCourseNavItemState, _Props::completed};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub articles: Vec<BBCourseNavArticle>,
    #[prop_or_default]
    pub classes: Classes,
}

#[styled_component(BBCourseNav)]
pub fn component(props: &Props) -> Html {
    let class = classes!(props.classes.clone(), "list-group",);

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

pub struct BBCourseNavArticleBuilder {
    completed: bool,
    title: Option<AttrValue>,
    current: bool,
    children: Option<Vec<BBCourseNavArticle>>,
    state: BBCourseNavItemState,
}

impl BBCourseNavArticleBuilder {
    pub fn new() -> Self {
        Self {
            completed: false,
            title: None,
            current: false,
            children: None,
            state: BBCourseNavItemState::Available,
        }
    }

    pub fn completed(mut self, new_completed: bool) -> Self {
        self.completed = new_completed;
        self
    }

    pub fn title(mut self, new_title: impl Into<AttrValue>) -> Self {
        self.title = Some(new_title.into());
        self
    }

    pub fn current(mut self, new_current: bool) -> Self {
        self.current = new_current;
        self
    }

    pub fn children(mut self, new_children: Vec<BBCourseNavArticle>) -> Self {
        self.children = Some(new_children);
        self
    }

    pub fn state(mut self, new_state: BBCourseNavItemState) -> Self {
        self.state = new_state;
        self
    }

    pub fn build(self) -> Option<BBCourseNavArticle> {
        self.title.map(|title| {
            BBCourseNavArticle {
                completed: self.completed,
                title: title,
                current: self.current,
                children: self.children,
                state: self.state,
            }
        })
    }
}
