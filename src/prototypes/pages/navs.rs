use crate::foundations::container::{BBContainer, BBContainerMargin};
use crate::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle, BBCourseNavArticleBuilder};
use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use yew::prelude::*;

#[function_component(Navs)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                {"Navigations"}
            </BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Course Nav"}</BBTitle>
            <BBCourseNav
                articles={course_nave_articles()}
            />
        </BBContainer>
    }
}

fn course_nave_articles() -> Option(Vec<BBCourseNavArticle>) {
    Some(vec![
        BBCourseNavArticleBuilder::new()
            .title("Introdution")
            .completed(true)
            .build()?,
        BBCourseNavArticleBuilder::new()
            .title("Routing")
            .children(
                BBCourseNavArticleBuilder::new()
                    .title("Introduction to routing")
                    .build()?,
            )
            .build()?,
    ])
}
