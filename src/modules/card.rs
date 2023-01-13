use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{elements::{
    button::{BBButton, BBButtonType},
    text::BBText,
    title::{BBTitle, BBTitleLevel},
}, foundations::tags::Tags};

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub title_level: BBTitleLevel,
    pub title: AttrValue,
    pub text: Option<AttrValue>,
    pub internal_link: Option<T>,
    #[prop_or_else(|| BBCardType::Simple)]
    pub card_type: BBCardType,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub debug: bool,
    #[prop_or_default]
    pub debug_name: AttrValue,
    #[prop_or_default]
    pub classes: Classes,
}

#[styled_component(BBCard)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let class = style!(
        r#"
        width: 18rem;
    "#
    )
    .unwrap();
    let card_type = props.card_type;

    html! {
        <div class={classes!("card", class, "mx-1", "my-1", props.classes.clone())}>
            {
                card_type.render(props)
            }
        </div>
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBCardType {
    Simple,
    CallToAction,
}

impl BBCardType {
    pub fn render<T: Routable + 'static>(&self, props: &Props<T>) -> Html {
        match self {
            BBCardType::Simple => self.simple(props),
            BBCardType::CallToAction => self.call_to_action(props),
        }
    }

    fn simple<T: Routable + 'static>(&self, props: &Props<T>) -> Html {
        html! {
            <div class="card-body mt-5 me-5">
                {
                    if props.internal_link.is_some() {
                        html! {
                            <Link<T> to={props.internal_link.clone().unwrap()}>
                                <BBTitle level={props.title_level} classes={classes!("card-title")}>{props.title.clone()}</BBTitle>
                            </Link<T>>
                        }
                    } else {
                        html! {
                            <BBTitle level={props.title_level} classes={classes!("card-title")}>{props.title.clone()}</BBTitle>
                        }
                    }
                }
                <BBText classes="card-text">{props.text.clone()}</BBText>
            </div>
        }
    }

    fn call_to_action<T: Routable + 'static>(&self, props: &Props<T>) -> Html {
        let onclick = {
            let debug = props.debug;
            let debug_name = props.debug_name.clone();
            let cb = props.onclick.clone();

            Callback::from(move |_: ()| {
                if debug {
                    log!(format!("Call to action clicked from {debug_name}"));
                }

                cb.emit(());
            })
        };

        html! {
            <div class="card-body text-center">
                <div class="h-25"></div>
                <BBButton
                    {onclick}
                    button_type={BBButtonType::PrimaryLight}
                    debug={props.debug}
                    debug_name={props.debug_name.clone()} >
                    {props.title.clone()}
                </BBButton>
            </div>
        }
    }
}

