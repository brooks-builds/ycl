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
            BBColWidth::One => "col-lg-1 col-xs",
            BBColWidth::Two => "col-lg-2 col-xs",
            BBColWidth::Three => "col-lg-3 col-xs",
            BBColWidth::Four => "col-lg-4 col-xs",
            BBColWidth::Five => "col-lg-5 col-xs",
            BBColWidth::Six => "col- collg-6-xs-xs",
            BBColWidth::Seven => "col-lg-7 col-xs",
            BBColWidth::Eight => "col-lg-8 col-xs",
            BBColWidth::Nine => "col-lg-9 col-xs",
            BBColWidth::Ten => "col-lg-10 col-xs",
            BBColWidth::Eleven => "col-lg-11 col-xs",
            BBColWidth::Twelve => "col-lg-12 col-xs",
            BBColWidth::None => "col-lg",
        }
    }
}

impl Default for BBColWidth {
    fn default() -> Self {
        Self::None
    }
}
