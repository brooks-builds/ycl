use crate::elements::button::{BBButton, BBButtonType};
use crate::elements::image::BBImage;
use crate::elements::title::{BBTitle, BBTitleLevel};
use crate::elements::youtube_video::BBYouTubeVideo;
use crate::foundations::align_text::AlignText;
use crate::foundations::column::BBCol;
use crate::foundations::container::BBContainer;
use crate::foundations::row::BBRow;
use crate::modules::hero::{BBHero, BBHeroLeftMedia};
use crate::modules::lms_promo::BBLmsPromo;
use yew::prelude::*;

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
                media={BBHeroLeftMedia::LeftMedia(hero_image())}
            />
            <BBTitle level={BBTitleLevel::Two}>{"Contact Banner"}</BBTitle>
            <BBHero
                text="Contact us to learn if our method of online education is right for you."
                title="Get in Touch"
                main={
                    html! {
                        <BBContainer classes={AlignText::Center.class()}>
                            <BBButton button_type={BBButtonType::PrimaryLight}>{"Contact Brooks"}</BBButton>
                        </BBContainer>
                    }
                }
                subtitle="This is a subtitl"
            />
            <BBTitle level={BBTitleLevel::Two}>{"Course Details"}</BBTitle>
            <BBHero
                text="Learn how to build web applications using the frontend framework Yew.rs. This framework is modeled after React, so should be familiar if you've done web programming before. Comes with the benefit of not needing to use JavaScript"
                title="$99"
                main={main_section()}
                media={BBHeroLeftMedia::LeftMedia(media())}
                subtitle="Web Development with Yew.rs"
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
