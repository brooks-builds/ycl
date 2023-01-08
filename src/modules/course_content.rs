use std::ops::Deref;

use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::HtmlElement;
use yew::{function_component, html, use_effect, use_state, AttrValue, Html, Properties};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = marked)]
    fn parse(markdown: &str) -> String;
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| true)]
    pub have_access: bool,
    pub course: AttrValue,
}

#[function_component(BBCourseContent)]
pub fn component(props: &Props) -> Html {
    let course = props.course.clone();
    let mounted = use_state(|| false);

    use_effect(move || {
        let return_closure = || {};

        if !mounted.deref() {
            let document = gloo::utils::document();

            let element = if let Ok(element) = document.query_selector("#course-content") {
                element
            } else {
                return return_closure;
            };

            if let Some(element) = element {
                let html_element = element.unchecked_into::<HtmlElement>();

                html_element.set_inner_html(&parse(course.deref()));
                mounted.set(true);
            }
        }

        return_closure
    });

    html! {
        <div id="course-content">

        </div>
    }
}
