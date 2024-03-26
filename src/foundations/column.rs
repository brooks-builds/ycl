use ::yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| BBColWidth::None)]
    pub width: BBColWidth,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(BBCol)]
pub fn component(props: &Props) -> Html {
    let class = props.width.class();

    html! {
        <div class={classes!(class, props.classes.clone())}>
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
            BBColWidth::Six => "col-xm-6",
            BBColWidth::Seven => "col-7",
            BBColWidth::Eight => "col-8",
            BBColWidth::Nine => "col-9",
            BBColWidth::Ten => "col-10",
            BBColWidth::Eleven => "col-11",
            BBColWidth::Twelve => "col-12",
            BBColWidth::None => "col-sm",
        }
    }
}

impl Default for BBColWidth {
    fn default() -> Self {
        Self::None
    }
}
