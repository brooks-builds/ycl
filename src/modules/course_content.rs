use crate::elements::button::{BBButton, BBButtonStyle};
use crate::foundations::container::BBContainer;
use crate::foundations::row::BBRow;
use std::ops::Deref;
use stylist::yew::styled_component;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::HtmlElement;
use yew::{html, use_effect, AttrValue, Callback, Html, Properties};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = marked)]
    fn parse(markdown: &str) -> String;

    #[wasm_bindgen(js_namespace = Prism, js_name = highlightAll)]
    fn highlight_all();
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| true)]
    pub have_access: bool,
    #[prop_or_default]
    pub logged_in: bool,
    pub course: AttrValue,
    #[prop_or_default]
    pub onclick_purchase: Callback<()>,
}

#[styled_component(BBCourseContent)]
pub fn component(props: &Props) -> Html {
    let course = props.course.clone();

    use_effect(move || {
        let return_closure = || {};

        let document = gloo::utils::document();

        let element = if let Ok(element) = document.query_selector("#course-content") {
            element
        } else {
            return return_closure;
        };

        if let Some(element) = element {
            let html_element = element.unchecked_into::<HtmlElement>();

            html_element.set_inner_html(&parse(course.deref()));

            highlight_all();
        }

        return_closure
    });

    let onclick_purchase = props.onclick_purchase.clone();
    let onclick = Callback::from(move |()| {
        onclick_purchase.emit(());
    });

    html! {
        <BBContainer>
            <BBRow classes="ms-auto">
                {
                    if !props.have_access {
                        let message = if props.logged_in {
                    "Purchase to access"
                } else {
                    "Log in to purchase"
                };
                        Some(html! {
                            <BBButton button_style={BBButtonStyle::PrimaryLight} {onclick}>{message}</BBButton>
                        })
                    } else {
                        None
                    }
                }
            </BBRow>
            <BBRow>
                <div id="course-content"></div>
            </BBRow>
        </BBContainer>
    }
}
