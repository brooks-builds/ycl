use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub width: Option<BBColWidth>,
}

#[function_component(BBCol)]
pub fn component(props: &Props) -> Html {
    let class = props.width.clone().unwrap_or_default().class();

    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BBColWidth {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    None,
}

impl BBColWidth {
    pub fn class(&self) -> &'static str {
        match self {
            BBColWidth::One => "col-1",
            BBColWidth::Two => "col-2",
            BBColWidth::Three => "col-3",
            BBColWidth::Four => "col-4",
            BBColWidth::Five => "col-5",
            BBColWidth::Six => "col-6",
            BBColWidth::Seven => "col-7",
            BBColWidth::Eight => "col-8",
            BBColWidth::Nine => "col-9",
            BBColWidth::Ten => "col-10",
            BBColWidth::Eleven => "col-11",
            BBColWidth::Twelve => "col-12",
            BBColWidth::None => "col",
        }
    }
}

impl Default for BBColWidth {
    fn default() -> Self {
        Self::None
    }
}
