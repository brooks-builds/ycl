use crate::{
    elements::{
        button::{BBButton, BBButtonType},
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::tags::Tags,
};
use gloo::console::log;
use stylist::{css, style, yew::styled_component, Style};
use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

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
        {
            wrap_in_link(
                props.internal_link.clone(),
                html! {
                    <div class={classes!("card", class, "mx-1", "my-1", props.classes.clone())}>
                        {
                            card_type.render(props)
                        }
                    </div>
                }
            )
        }
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
                <BBTitle level={props.title_level} classes={classes!("card-title")}>{props.title.clone()}</BBTitle>
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
                <div class={Style::new(css!("height: 4rem;")).unwrap()}></div>
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

fn wrap_in_link<T: Routable + 'static>(link: Option<T>, children: VNode) -> Html {
    if let Some(link) = link {
        html! {
            <Link<T> to={link} classes="card-link">
                {children}
            </Link<T>>
        }
    } else {
        html! {
            {children}
        }
    }
}
