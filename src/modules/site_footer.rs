use super::nav::nav_footer::BBNavFooter;
use super::nav::navbar_link::BBNavbarLink;
use crate::{
    elements::icon::{BBIcon, BBIconSize, BBIconType},
    foundations::{column::BBCol, container::BBContainer, row::BBRow},
};
use chrono::{Datelike, Utc};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::Routable;

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: Routable + 'static,
{
    pub left_links: Vec<BBNavbarLink<T>>,
    pub right_links: Vec<BBNavbarLink<T>>,
}

#[styled_component(BBSiteFooter)]
pub fn component<T: Routable + 'static>(props: &Props<T>) -> Html {
    let now = Utc::now();
    let year = now.year();

    html! {
        <BBContainer classes={classes!("bg-secondary", Style::new(css!("margin: 1rem 0;")).unwrap())}>
           <BBRow classes="pt-5">
              <BBCol classes="text-center">
                  <BBIcon icon_type={BBIconType::Mark} size={BBIconSize::Small} />
              </BBCol>
           </BBRow>
           <BBRow>
              <BBCol classes={classes!("text-center", Style::new(css!("margin: 1rem 0;")).unwrap())}>
                {format!("©{year} Brooks Builds LLC. All rights reserved")}
              </BBCol>
            </BBRow>
            <BBRow classes="text-center pb-5">
                <BBCol>
                    <a href="https://twitter.com/brooks_patton" >
                        <BBIcon icon_type={BBIconType::Twitter} size={BBIconSize::Small} />
                    </a>
                    <a
                        href="https://twitch.tv/brookzerker"
                        class={
                            Style::new(css!(
                                    r#"position: relative;"#)).unwrap()
                        }
                    >
                        <BBIcon icon_type={BBIconType::Twitch} size={BBIconSize::Small} />
                    </a>
                    <a href="https://www.youtube.com/channel/UCT1-XRVnJA-wws2bfbLbFcQ" >
                        <BBIcon icon_type={BBIconType::YouTubeSmall} size={BBIconSize::Small} />
                    </a>
                </BBCol>
           </BBRow>
           <BBRow>
                <BBNavFooter<T>
                    left_links={props.left_links.clone()}
                    right_links={props.right_links.clone()}
                />
           </BBRow>
        </BBContainer>
    }
}
