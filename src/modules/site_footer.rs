use super::nav::nav_footer::BBNavFooter;
use super::nav::navbar_link::BBNavbarLink;
use crate::modules::icons_row::{BBIconsRow, BBIconsRowList};
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
    let icons = vec![
        BBIconsRowList::new(BBIconType::Twitter, "https://twitter.com/brooks_patton"),
        BBIconsRowList::new(BBIconType::Twitch, "https://twitch.tv/brookzerker"),
        BBIconsRowList::new(
            BBIconType::YouTubeSmall,
            "https://www.youtube.com/@BrooksBuilds",
        ),
    ];

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
            <BBIconsRow {icons}/>
           <BBRow>
                <BBNavFooter<T>
                    left_links={props.left_links.clone()}
                    right_links={props.right_links.clone()}
                />
           </BBRow>
        </BBContainer>
    }
}
