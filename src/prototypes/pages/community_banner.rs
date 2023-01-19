use yew::prelude::*;

use crate::{modules::hero::{BBHero, BBHeroLeftMedia}, foundations::{container::BBContainer, row::BBRow, column::BBCol, align_text::AlignText}, elements::{button::{BBButton, BBButtonType}, image::BBImage}};

#[function_component(CommunityBanner)]
pub fn component() -> Html {
    html! {
        <BBHero 
            text="Courses built to be completed, and with a community."
            title="Built for People"
            main={main_section()}
            media={BBHeroLeftMedia::Image(hero_image())}
        />
    }
}

fn main_section() -> Html {
    html! {
        <BBContainer>
            <BBRow>
                <BBCol classes={AlignText::Center.class()}>
                    <BBButton button_type={BBButtonType::PrimaryLight} classes="me-1">{"Get Started"}</BBButton>
                    <BBButton button_type={BBButtonType::PrimaryLight}>{"Course Docs"}</BBButton>
                </BBCol>
            </BBRow>
        </BBContainer>
    }
}

fn hero_image() -> Html {
    html! {
        <BBImage src="/code.png" alt="Some code we found on the internet" />
    }
}
