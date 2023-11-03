use crate::foundations::container::{BBContainer, BBContainerMargin};
use crate::foundations::errors::BBError;
use crate::modules::nav::course_nav::{BBCourseNav, BBCourseNavArticle, BBCourseNavArticleBuilder};
use crate::prototypes::router::Route;
use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "enableTooltips")]
    fn enable_tooltips();
}

#[function_component(Navs)]
pub fn component() -> Html {
    use_effect_once(|| {
        enable_tooltips();

        || {}
    });

    let onclick = Callback::from(|id: AttrValue| {
        gloo::console::info!("Article with id ", id.as_str(), "clicked");
    });

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                {"Navigations"}
            </BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Course Nav"}</BBTitle>
            <BBCourseNav<Route>
                articles={course_nave_articles().unwrap()}
                {onclick}
            />
        </BBContainer>
    }
}

fn course_nave_articles() -> Result<Vec<BBCourseNavArticle<Route>>, BBError> {
    Ok(vec![
        BBCourseNavArticleBuilder::new()
            .title("Introdution")
            .completed(true)
            .to(Route::Navs)
            .id("1")
            .build()?,
        BBCourseNavArticleBuilder::new()
            .title("Routing")
            .children(vec![BBCourseNavArticleBuilder::new()
                .title("Introduction to routing")
                .to(Route::Navs)
                .id("3")
                .build()?])
            .to(Route::Navs)
            .id("2")
            .build()?,
        BBCourseNavArticleBuilder::new()
            .title("Context")
            .to(Route::Navs)
            .id("4")
            .build()?,
    ])
}
