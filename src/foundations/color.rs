use yew::{classes, Classes};

#[derive(PartialEq)]
pub enum BBColor {
    Primary,
    Success,
}

impl BBColor {
    pub fn bg_color(&self) -> Classes {
        classes!(match self {
            Self::Primary => "bg-primary",
            Self::Success => "bg-success",
        })
    }
}
