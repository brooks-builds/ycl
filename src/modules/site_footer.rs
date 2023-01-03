use yew::prelude::*;

use crate::{
    elements::icon::{BBIcon, BBIconSize, BBIconType},
    foundations::{column::BBCol, container::BBContainer, row::BBRow},
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(BBSiteFooter)]
pub fn component(_props: &Props) -> Html {
    html! {
        <BBContainer classes="bg-secondary">
           <BBRow>
              <BBCol classes="text-center">
                  <BBIcon icon_type={BBIconType::Mark} size={BBIconSize::Small} />
              </BBCol>
           </BBRow>
           <BBRow>
              <BBCol classes="text-center">
                {"© Brooks Builds LLC. All rights reserved"}
              </BBCol>
            </BBRow>
            <BBRow classes="text-center">
                <BBCol>
                    <a href="https://twitter.com/brooks_patton" >
                        <BBIcon icon_type={BBIconType::Twitter} size={BBIconSize::Small} />
                    </a>
                    <a href="https://twitch.tv/brookzerker" >
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
