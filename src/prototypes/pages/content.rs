use crate::foundations::container::{BBContainer, BBContainerMargin};
use crate::modules::course_content::BBCourseContent;
use crate::{
    elements::title::{BBTitle, BBTitleLevel},
    foundations::align_text::AlignText,
};
use indoc::indoc;
use yew::prelude::*;

#[function_component(Content)]
pub fn component() -> Html {
    let course = indoc! {"
        # Introduction to Yew Routing

        ## What is Routing

        When we mention routing in single page applications like Yew.rs we are talking about the ability to change pages without re-loading the content or making any requests for the base HTML. This makes loading pages super fast and the experience is like a app loaded locally, because it is.

        ## Code

        ### Creating the Route enum

        ```rust
        use yew_router::prelude::*;

        #[derive(Clone, Routable, PartialEq)]
        pub enum Route {
            #[at(\"/\")]
            Home,
        }
        
        ```

        As you can see above, the route is simply an enum. Enums make sense because we can only be visiting one page at a time as far as the app is concerned.
    "};

    let onclick_purchase = Callback::from(|event| {
        gloo::console::log!("the purchase button was clicked");
    });

    html! {
        <BBContainer margin={BBContainerMargin::Normal}>
            <BBTitle level={BBTitleLevel::One} align={AlignText::Center}>
                {"Content"}
            </BBTitle>
            <BBTitle level={BBTitleLevel::Two}>{"Course Content"}</BBTitle>
            <BBCourseContent
                {course}
                {onclick_purchase}
                have_access={false}
            />
        </BBContainer>
    }
}
