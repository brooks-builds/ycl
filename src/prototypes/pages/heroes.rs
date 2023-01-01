use yew::prelude::*;

use crate::{
    elements::{
        title::{BBTitle, BBTitleLevel},
        youtube_video::BBYouTubeVideo,
    },
    foundations::{align_text::AlignText, container::BBContainer, row::BBRow},
    modules::hero::BBHero,
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHeros)]
pub fn component(_props: &Props) -> Html {
    html! {
        <BBContainer full_width={true}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>{"Heroes"}</BBTitle>
            <BBRow>
               <BBTitle level={BBTitleLevel::Two} align={AlignText::Left}>{"Billboard"}</BBTitle>
               <BBHero
                    title="Brooks Builds Learners!"
                    text="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor!" />
                <BBTitle level={BBTitleLevel::Two} align={AlignText::Left}>{"Section Hero"}</BBTitle>
                <BBHero
                    title="This is a section"
                    text="We can embed whatever we want in this!"
                    main={
                        html! {
                            <BBYouTubeVideo
                                src="https://www.youtube-nocookie.com/embed/5PB9UDOIuGk"
                                title="Introduction to Yew Trailer"
                                align={AlignText::Center} />
                        }
                    }
                    title_level={BBTitleLevel::Three} />
            </BBRow>
        </BBContainer>
    }
}
