use yew::prelude::*;
use crate::elements::title::BBTitleLevel;
use crate::elements::youtube_video::BBYouTubeVideo;
use crate::foundations::align_text::AlignText;
use crate::foundations::container::{BBContainer, BBContainerMargin};
use crate::modules::hero::BBHero;
use crate::modules::lms_promo::BBLmsPromo;

#[function_component(LmsPromo)]
pub fn component() -> Html {
    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBLmsPromo 
                title="Learning Management"
                title_level={BBTitleLevel::One}
                description="The Brooks Builds Learning Management System is a custom platform to help you learn the skills that you need to learn to get to where you want to go in your engineering career."
                media={media()}
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
