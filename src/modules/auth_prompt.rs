#![allow(non_camel_case_types)]

use crate::{
    elements::{
        external_link::BBLink,
        text::BBText,
        title::{BBTitle, BBTitleLevel},
    },
    foundations::{align_text::AlignText, column::BBCol, container::BBContainer, row::BBRow},
    modules::icons_row::{BBIconsRow, BBIconsRowList},
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub auth_icons: Vec<BBIconsRowList>,
    #[prop_or_default]
    pub username_password: Option<AttrValue>,
    #[prop_or_else(|| BBTitleLevel::One)]
    pub title_level: BBTitleLevel,
    #[prop_or_default]
    pub register: bool,
}

#[function_component(BBAuthPrompt)]
pub fn component(props: &Props) -> Html {
    html! {
        <BBContainer>
            <BBRow>
                <BBCol>
                    <BBTitle
                        level={props.title_level}
                        align={AlignText::Center}
                    >
                        {
                            if props.register {
                                "Register Account"
                            } else {
                                "Login"
                            }
                        }
                    </BBTitle>
                </BBCol>
            </BBRow>
            <BBRow>
                <BBCol>
                    <BBText align={AlignText::Center}>
                        {
                            if props.register {
                                "Create new account using social links or username/password"
                            } else {
                                "Login to your account"
                            }
                        }
                    </BBText>
                </BBCol>
            </BBRow>
            <BBIconsRow icons={props.auth_icons.clone()} />
            <BBRow>
                <BBCol>
                    <BBText align={AlignText::Center}>{"OR"}</BBText>
                </BBCol>
            </BBRow>

            <BBRow>
                <BBCol
                    classes={classes!(AlignText::Center.class())}
                >
                    <BBLink href="https://twitch.tv/brookzerker" button={true}>{"Username / Password"}</BBLink>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}
