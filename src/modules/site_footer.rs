use yew::prelude::*;

use crate::{
    elements::icon::{BBIcon, BBIconSize, BBIconType},
    foundations::{column::BBCol, container::BBContainer, row::BBRow},
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(BBSiteFooter)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer full_width={true}>
           <BBRow classes="text-center bg-secondary">
              <BBCol>
                  <BBIcon icon_type={BBIconType::Mark} size={BBIconSize::Small} />
              </BBCol>
           </BBRow>
        </BBContainer>
    }
}
