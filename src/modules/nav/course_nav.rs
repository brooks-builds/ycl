use crate::{elements::course_nav_item::BBCourseNavItem, foundations::errors::BBError};
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<R>
where
    R: Routable + 'static,
{
    pub articles: Vec<BBCourseNavArticle<R>>,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub onclick: Callback<AttrValue>,
}

#[styled_component(BBCourseNav)]
pub fn component<R: Routable + 'static>(props: &Props<R>) -> Html {
    let class = classes!(props.classes.clone(), "list-group", "course-nav");
    let props_onclick = props.onclick.clone();
    let onclick = Callback::from(move |event: MouseEvent| {
        let Some(target) = event.target() else { return };
        let html_element = target.unchecked_into::<HtmlElement>();
        let Some(id )= html_element.get_attribute("data-id") else { return };

        props_onclick.emit(id.into());
    });

    html! {
        <ul {class} {onclick}>
            {
                props
                    .articles
                    .clone()
                    .into_iter()
                    .map(move |article| {
                        create_course_nav_item(article)
                    })
                    .collect::<Html>()
            }
        </ul>
    }
}

fn create_course_nav_item<R: Routable + 'static>(article: BBCourseNavArticle<R>) -> Html {
    html! {
        <BBCourseNavItem<R>
            completed={article.completed}
            title={article.title}
            current={article.current}
            children={article.children}
            to={article.to}
            preview={article.preview}
            id={article.id}
        />
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BBCourseNavArticle<R>
where
    R: Routable + 'static,
{
    pub completed: bool,
    pub title: AttrValue,
    pub current: bool,
    pub children: Option<Vec<BBCourseNavArticle<R>>>,
    pub to: Option<R>,
    pub preview: bool,
    pub id: AttrValue,
}

impl<R: Routable + 'static> BBCourseNavArticle<R> {
    pub fn active_class(&self) -> Option<&'static str> {
        if self.current {
            Some("active")
        } else {
            None
        }
    }
}

pub struct BBCourseNavArticleBuilder<R>
where
    R: Routable + 'static,
{
    completed: bool,
    title: Option<AttrValue>,
    current: bool,
    children: Option<Vec<BBCourseNavArticle<R>>>,
    to: Option<R>,
    preview: bool,
    id: Option<AttrValue>,
}

impl<R: Routable + 'static> BBCourseNavArticleBuilder<R> {
    pub fn new() -> Self {
        Self {
            completed: false,
            title: None,
            current: false,
            children: None,
            to: None,
            preview: false,
            id: None,
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

    pub fn children(mut self, new_children: Vec<BBCourseNavArticle<R>>) -> Self {
        self.children = Some(new_children);
        self
    }

    pub fn to(mut self, to: R) -> Self {
        self.to = Some(to);
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = preview;
        self
    }

    pub fn id(mut self, id: impl Into<AttrValue>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<BBCourseNavArticle<R>, BBError> {
        Ok(BBCourseNavArticle {
            completed: self.completed,
            title: self
                .title
                .ok_or(BBError::CourseNavItemArticleBuilder("missing title"))?,
            current: self.current,
            children: self.children,
            to: self.to,
            preview: self.preview,
            id: self
                .id
                .ok_or(BBError::CourseNavItemArticleBuilder("missing id"))?,
        })
    }
}

impl<R: Routable + 'static> Default for BBCourseNavArticleBuilder<R> {
    fn default() -> Self {
        Self::new()
    }
}
