use crate::{
    elements::icon::{BBIcon, BBIconSize, BBIconType},
    foundations::{column::BBCol, container::BBContainer, row::BBRow},
};
use chrono::{Datelike, Utc};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[styled_component(BBSiteFooter)]
pub fn component(_props: &Props) -> Html {
    let now = Utc::now();
    let year = now.year();

    html! {
        <BBContainer classes={classes!("bg-secondary", Style::new(css!("padding: 1rem 0;")).unwrap())}>
           <BBRow>
              <BBCol classes="text-center">
                  <BBIcon icon_type={BBIconType::Mark} size={BBIconSize::Small} />
              </BBCol>
           </BBRow>
           <BBRow>
              <BBCol classes={classes!("text-center", Style::new(css!("margin: 1rem 0;")).unwrap())}>
                {format!("©{year} Brooks Builds LLC. All rights reserved")}
              </BBCol>
            </BBRow>
            <BBRow classes="text-center">
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
        </BBContainer>
    }
}
