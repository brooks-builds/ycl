use yew::prelude::*;

use crate::{
    elements::{
        image::BBImage,
        title::{BBTitle, BBTitleLevel},
        youtube_video::BBYouTubeVideo,
    },
    foundations::{align_text::AlignText, container::BBContainer, row::BBRow},
    modules::hero::{BBHero, BBHeroLeftMedia},
};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(PHeros)]
pub fn component(_props: &Props) -> Html {
    html! {
        <BBContainer>
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
                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Left}>{"Section hero with left image"}</BBTitle>
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
                        title_level={BBTitleLevel::Three}
                        media={BBHeroLeftMedia::Image(html! { <BBImage src="/code.png" alt="Some code" /> })}
                    />

                    <BBTitle level={BBTitleLevel::Two} align={AlignText::Left}>{"Section hero with left video"}</BBTitle>
                    <BBHero
                        title="This is a section"
                        text="Yay, we can have a video to the left!"
                        title_level={BBTitleLevel::Three}
                        media={BBHeroLeftMedia::Image(html! { <BBYouTubeVideo
                                    src="https://www.youtube-nocookie.com/embed/5PB9UDOIuGk"
                                    title="Introduction to Yew Trailer"
                                    align={AlignText::Center} /> })}
                    />
            </BBRow>
        </BBContainer>
    }
}
