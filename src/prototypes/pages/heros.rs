use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::foundations::align_text::AlignText;
use crate::foundations::container::BBContainer;
use crate::modules::hero::{BBHero, BBHeroLeftMedia};
use yew::prelude::*;
use crate::modules::lms_promo::BBLmsPromo;
use crate::elements::youtube_video::BBYouTubeVideo;
use crate::elements::image::BBImage;
use crate::elements::button::{BBButton, BBButtonType};
use crate::foundations::row::BBRow;
use crate::foundations::column::BBCol;

#[function_component(Heros)]
pub fn component() -> Html {
    html! {
        <BBContainer>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Heros"}</BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Hero"}</BBTitle>
            <BBHero
                title="Brooks Builds Learners"
                text="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor!"
                title_level={BBTitleLevel::Three}
            />
            <BBTitle level={BBTitleLevel::Two}>{"LMS Promo"}</BBTitle>
            <BBLmsPromo 
                title="Learning Management"
                title_level={BBTitleLevel::Three}
                description="The Brooks Builds Learning Management System is a custom platform to help you learn the skills that you need to learn to get to where you want to go in your engineering career."
                media={media()}
            />
            <BBTitle level={BBTitleLevel::Two}>{"Community Banner"}</BBTitle>
            <BBHero 
                text="Courses built to be completed, and with a community."
                title="Built for People"
                main={main_section()}
                media={BBHeroLeftMedia::Image(hero_image())}
            />
        </BBContainer>
    }
}

fn media() -> Html {
    html! {
        <BBYouTubeVideo
            src="https://www.youtube.com/embed/kEosrMDPUPA"
            title="Introduction to Yew.rs"
            align={AlignText::Center}
            width={560}
            height={315}
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
