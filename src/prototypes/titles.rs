use crate::elements::{
    accordian_item::BBAccordianItem,
    title::{BBTitle, BBTitleLevel},
};
use yew::{html, virtual_dom::VNode};

pub fn add_titles(accordian_items: &mut Vec<VNode>, parent_id: String) {
    accordian_items.push(html! {
        <BBAccordianItem title="Title One" title_level={BBTitleLevel::Two} id="title_one" parent_id={parent_id.clone()}>
            <BBTitle level={BBTitleLevel::One}>{"Title One"}</BBTitle>
        </BBAccordianItem>
    });

    accordian_items.push(html! {
        <BBAccordianItem title="Title Two" title_level={BBTitleLevel::Two} id="title_two" parent_id={parent_id.clone()}>
            <BBTitle level={BBTitleLevel::Two}>{"Title Two"}</BBTitle>
        </BBAccordianItem>
    });

    accordian_items.push(html! {
        <BBAccordianItem title="Title Three" title_level={BBTitleLevel::Two} id="title_three" parent_id={parent_id.clone()}>
            <BBTitle level={BBTitleLevel::Three}>{"Title Three"}</BBTitle>
        </BBAccordianItem>
    });

    accordian_items.push(html! {
        <BBAccordianItem title="Title Four" title_level={BBTitleLevel::Two} id="title_four" parent_id={parent_id.clone()}>
            <BBTitle level={BBTitleLevel::Four}>{"Title Four"}</BBTitle>
        </BBAccordianItem>
    });

    accordian_items.push(html! {
        <BBAccordianItem title="Title Five" title_level={BBTitleLevel::Two} id="title_five" parent_id={parent_id.clone()}>
            <BBTitle level={BBTitleLevel::Five}>{"Title Five"}</BBTitle>
        </BBAccordianItem>
    });

    accordian_items.push(html! {
        <BBAccordianItem title="Title Six" title_level={BBTitleLevel::Two} id="title_six" parent_id={parent_id.clone()}>
            <BBTitle level={BBTitleLevel::Six}>{"Title Six"}</BBTitle>
        </BBAccordianItem>
    });
}
