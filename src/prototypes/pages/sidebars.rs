use crate::elements::course_nav_item::BBCourseNavItemState;
use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::foundations::align_text::AlignText;
use crate::modules::course_nav::{BBCourseNav, BBCourseNavArticle};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = enableTooltips)]
    fn enable_tooltips();
}

#[function_component(PSidebars)]
pub fn component() -> Html {
    let articles = vec![
        BBCourseNavArticle {
            children: None,
            completed: true,
            current: false,
            title: "First article".into(),
            state: BBCourseNavItemState::Available,
        },
        BBCourseNavArticle {
            children: None,
            completed: false,
            current: true,
            title: "Current article".into(),
            state: BBCourseNavItemState::Available,
        },
        BBCourseNavArticle {
            children: Some(vec![
                BBCourseNavArticle {
                    children: None,
                    completed: false,
                    current: false,
                    title: "first child".into(),
                    state: BBCourseNavItemState::Available,
                },
                BBCourseNavArticle {
                    children: None,
                    completed: false,
                    current: false,
                    title: "second child".into(),
                    state: BBCourseNavItemState::Available,
                },
                BBCourseNavArticle {
                    children: None,
                    completed: false,
                    current: false,
                    title: "third child".into(),
                    state: BBCourseNavItemState::Available,
                },
            ]),
            completed: false,
            current: false,
            title: "This time with children".into(),
            state: BBCourseNavItemState::Available,
        },
        BBCourseNavArticle {
            children: None,
            completed: false,
            current: false,
            title: "This is not written yet".into(),
            state: BBCourseNavItemState::NotYetCreated,
        },
        BBCourseNavArticle {
            children: None,
            completed: false,
            current: false,
            title: "This needs to be purchased to view".into(),
            state: BBCourseNavItemState::NeedToPurchase,
        },
    ];

    use_effect_once(|| {
        enable_tooltips();

        || {}
    });

    html! {
        <main>
            <BBTitle
                level={BBTitleLevel::One}
                align={AlignText::Center}>
                {"Sidebars"}
            </BBTitle>
            <BBTitle
                level={BBTitleLevel::Two} >
                {"Course Nav"}
            </BBTitle>
            <BBCourseNav {articles} />
        </main>
    }
}
